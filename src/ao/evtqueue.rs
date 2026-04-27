use crate::ao::ConstNullable;

/// evtqueue.rs
///
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

// fns for disabling interrupts
unsafe extern "C" {
    fn c_enter_critical() -> u32;
    fn c_exit_critical(state: u32);
}

pub enum QueueErr {
    /// Queue is full
    Full,
}

pub struct Mpsc<T, const N: usize> {
    buffer: UnsafeCell<[T; N]>,
    head: AtomicUsize,
    tail: AtomicUsize,
}

// manually tell the compiler, we promise this Queue is safe to share across
// interrupt priorities.
unsafe impl<T, const N: usize> Sync for Mpsc<T, N> where T: Send {}

impl<T, const N: usize> Mpsc<T, N>
where
    T: Send + Copy + ConstNullable,
{
    /// create new.
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([T::NONE; N]),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// push an event to the queue.
    pub fn push(&self, event: T) -> Result<(), QueueErr> {
        // disable irq, since events can be pushed to the queue in ISR context.
        let irq_state = unsafe { c_enter_critical() };
        let h = self.head.load(Ordering::Relaxed);
        let next_h = (h + 1) % N;

        if next_h == self.tail.load(Ordering::Acquire) {
            // exit CS before returning
            unsafe {
                c_exit_critical(irq_state);
            }
            return Err(QueueErr::Full);
        }

        unsafe {
            let buf_ptr = self.buffer.get();
            (*buf_ptr)[h] = event;
        }

        self.head.store(next_h, Ordering::Release);
        // enable irq again
        unsafe {
            c_exit_critical(irq_state);
        }
        Ok(())
    }

    /// pop an event from the queue.
    ///
    /// - it will return None if queue is empty.
    pub fn pop(&self) -> Option<T> {
        let t = self.tail.load(Ordering::Relaxed);

        if t == self.head.load(Ordering::Acquire) {
            return None;
        }

        // grab it from the cell
        // we know that no two AO's can pop from the queue simultaneously.
        let event = unsafe {
            let buf_ptr = self.buffer.get();
            let val = (*buf_ptr)[t];
            (*buf_ptr)[t] = T::NONE;
            val
        };

        let next_t = (t + 1) % N;
        self.tail.store(next_t, Ordering::Release);
        Some(event)
    }
}
