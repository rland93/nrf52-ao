use crate::ao;
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

pub(crate) trait Level {
    const EGU_PRIORITY: u8;
}

pub(crate) struct P7;
pub(crate) struct P6;
pub(crate) struct P5;
pub(crate) struct P3;

impl Level for P7 {
    const EGU_PRIORITY: u8 = 7;
}

impl Level for P6 {
    const EGU_PRIORITY: u8 = 6;
}

impl Level for P5 {
    const EGU_PRIORITY: u8 = 5;
}

impl Level for P3 {
    const EGU_PRIORITY: u8 = 3;
}

// a C function to trigger the appropriate EGU, given the
unsafe extern "C" {
    // hw_egu.c
    fn c_trigger_egu(prio: u8);
}

fn drain<L: Level>(aos: &[&dyn DispatchableAt<L>]) {
    loop {
        let mut did_work = false;
        for ao in aos {
            if ao.dispatch_one() {
                did_work = true;
            }
        }
        if !did_work {
            break;
        }
    }
}

/// called by EGU isr to dispatch pending events to waiting AOs of a given prio
///
/// if an invalid priority is passed in this is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn rust_egu_dispatcher(prio: u8) {
    // select the ao level to dispatch

    match prio {
        3 => drain(ao::PRIO_3_AOS),
        5 => drain(ao::PRIO_5_AOS),
        6 => drain(ao::PRIO_6_AOS),
        7 => drain(ao::PRIO_7_AOS),
        _ => {
            return;
        }
    };
}

pub fn trigger_egu(prio: u8) {
    unsafe {
        c_trigger_egu(prio);
    }
}

pub trait DispatchableAt<L: Level>: Sync {
    // attempt to pop and process one event.
    //
    // true if work was done, false if there's none left
    fn dispatch_one(&self) -> bool;
}

pub trait StateMachine: Send {
    type Event: Sync + Send + Copy + ao::ConstNullable;

    /// runs when event is popped
    fn handle_event(&mut self, event: Self::Event);
}

pub struct AoContext<T: StateMachine, L: Level, const Q_SIZE: usize> {
    queue: ao::evtqueue::Mpsc<T::Event, Q_SIZE>,
    is_active: AtomicBool,
    state_machine: UnsafeCell<T>,
    _prio: PhantomData<L>,
}

impl<T: StateMachine, L: Level, const Q_SIZE: usize> AoContext<T, L, Q_SIZE> {
    pub const fn new(state_machine: T) -> Self {
        let is_active = AtomicBool::new(false);
        let queue = ao::evtqueue::Mpsc::new();

        Self {
            queue: queue,
            is_active: is_active,
            state_machine: UnsafeCell::new(state_machine),
            _prio: PhantomData::<L>,
        }
    }

    pub fn post(&self, event: T::Event) {
        // push event to the queue
        match self.queue.push(event) {
            Ok(()) => {}
            Err(_e) => {}
        }

        // pend the event
        trigger_egu(L::EGU_PRIORITY);
    }
}

// dispatch_one is only ever called from one EGU level, therefore we can mutate
// the unsafe cell inside.
unsafe impl<T: StateMachine, L: Level, const Q_SIZE: usize> Sync for AoContext<T, L, Q_SIZE> {}

impl<T: StateMachine, L: Level, const Q_SIZE: usize> DispatchableAt<L> for AoContext<T, L, Q_SIZE> {
    // true if work was done; false if there's none left in this AO
    #[inline(never)]
    fn dispatch_one(&self) -> bool {
        match self.queue.pop() {
            Some(event) => {
                // object is now handling an event.
                self.is_active.store(true, Ordering::Release);

                // state machine is in an unsafecell. it is safe to acquire the
                // sm here, because we are dispatching to the appropriate
                // consumer, at the appropriate egu level, by the time we get
                // here. No two execution contexts can ever dispatch_one() on
                // the same AO concurrently, because dispatch_one() is only ever
                // triggered by an software interrupt (EGU).
                let sm = unsafe { &mut *self.state_machine.get() };
                sm.handle_event(event);

                // object is back to idle
                self.is_active.store(false, Ordering::Release);

                // return dispatched.
                true
            }
            None => false,
        }
    }
}
