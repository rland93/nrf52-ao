use crate::ao::objects::*;
use crate::ao::{ConstNullable, evtqueue::Mpsc};
use core::cell::UnsafeCell;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

// a C function to trigger the appropriate EGU, given the
unsafe extern "C" {
    // hw_egu.c
    fn c_trigger_egu(prio: u8);
}

/// called by EGU isr to dispatch pending events to waiting AOs of a given prio
///
/// if an invalid priority is passed in this is a no-op.
#[unsafe(no_mangle)]
pub extern "C" fn rust_egu_dispatcher(prio: u8) {
    // select the ao level to dispatch
    let aos_to_dispatch = match prio {
        3 => PRIO_3_AOS,
        5 => PRIO_5_AOS,
        6 => PRIO_6_AOS,
        7 => PRIO_7_AOS,
        _ => {
            return;
        }
    };

    // loop through all in this prio and dispatch each
    loop {
        let mut did_work = false;
        for ao in aos_to_dispatch {
            if ao.dispatch_one() {
                did_work = true;
            }
        }
        if !did_work {
            break;
        }
    }
}

/// priority levels
#[derive(Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Priority {
    _Low = 7,
    _Medium = 6,
    High = 5,
    _XHigh = 3,
}

impl Priority {
    pub fn trigger_egu(&self) {
        unsafe {
            c_trigger_egu(*self as u8);
        }
    }
}

pub trait Dispatchable: Sync {
    // attempt to pop and process one event.
    //
    // true if work was done, false if there's none left
    fn dispatch_one(&self) -> bool;
}

pub trait StateMachine: Send {
    type Event: Sync + Send + Copy + ConstNullable;

    /// runs when event is popped
    fn handle_event(&mut self, event: Self::Event);
}

pub struct AoContext<T: StateMachine, const Q_SIZE: usize> {
    queue: Mpsc<T::Event, Q_SIZE>,
    is_active: AtomicBool,
    priority: Priority,
    state_machine: UnsafeCell<T>,
}

impl<T: StateMachine, const Q_SIZE: usize> AoContext<T, Q_SIZE> {
    pub const fn new(priority: Priority, state_machine: T) -> Self {
        let is_active = AtomicBool::new(false);
        let queue = Mpsc::new();

        Self {
            queue: queue,
            is_active: is_active,
            priority: priority,
            state_machine: UnsafeCell::new(state_machine),
        }
    }

    pub fn post(&self, event: T::Event) {
        // push event to the queue
        match self.queue.push(event) {
            Ok(()) => {}
            Err(_e) => {}
        }

        // pend the event
        self.priority.trigger_egu();
    }
}

// dispatch_one is only ever called from one EGU level, therefore we can mutate
// the unsafe cell inside.
unsafe impl<T: StateMachine, const Q_SIZE: usize> Sync for AoContext<T, Q_SIZE> {}

impl<T: StateMachine, const Q_SIZE: usize> Dispatchable for AoContext<T, Q_SIZE> {
    // true if work was done; false if there's none left in this AO
    #[inline(never)]
    fn dispatch_one(&self) -> bool {
        match self.queue.pop() {
            Some(event) => {
                // object is now handling an event.
                self.is_active.store(true, Ordering::Release);

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
