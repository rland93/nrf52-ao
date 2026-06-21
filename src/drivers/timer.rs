use crate::ffi;
use core::cmp::Ordering;
use core::num::NonZeroU32;

pub const TICKS_PER_MS: u32 = 32768 / 1_000;
pub const TICKS_PER_S: u32 = 32768;

#[derive(Clone, Copy)]
pub struct TimerEntry {
    /// deadline (in ticks)
    deadline: u32,
    /// event id to post when this timer fires
    event_id: u8,
    /// recurring timer. None for one-shot.
    period: Option<NonZeroU32>,
    /// whether this entry is active
    active: bool,
}

/// object that handles scheduling future events on a timer
pub struct TimerScheduler<const N: usize> {
    entries: [TimerEntry; N],
    count: u8,
}

impl TimerEntry {
    const INACTIVE: TimerEntry = TimerEntry {
        deadline: 0,
        event_id: 0,
        period: None,
        active: false,
    };
}

impl<const N: usize> TimerScheduler<N> {
    pub const fn new() -> Self {
        Self {
            entries: [TimerEntry::INACTIVE; N],
            count: 0,
        }
    }

    pub fn start(&mut self) {
        unsafe {
            ffi::rtc::c_rtc_start();
        };
    }

    pub fn schedule_oneshot(&mut self, delay_ticks: u32, event_id: u8) -> Result<(), ()> {
        let now = self.now();
        let deadline = now.wrapping_add(delay_ticks);
        self.schedule_absolute(deadline, event_id, None)
    }

    pub fn schedule_recurring(
        &mut self,
        delay_ticks: u32,
        period_ticks: NonZeroU32,
        event_id: u8,
    ) -> Result<(), ()> {
        let now = self.now();
        let deadline = now.wrapping_add(delay_ticks);
        self.schedule_absolute(deadline, event_id, Some(period_ticks))
    }

    fn schedule_absolute(
        &mut self,
        deadline: u32,
        event_id: u8,
        period: Option<NonZeroU32>,
    ) -> Result<(), ()> {
        if self.count as usize >= N {
            return Err(());
        }

        // find first inactive slot
        let slot_idx = self.entries.iter().position(|e| !e.active).ok_or(())?;

        // retrieve nearest deadline
        let old_nearest = self.nearest_deadline();

        // populate new deadline into slot
        self.entries[slot_idx] = TimerEntry {
            deadline,
            event_id,
            period,
            active: true,
        };

        self.count += 1;

        // new nearest after adding
        let new_nearest = self.nearest_deadline().unwrap();

        // reprogram if the nearest changed
        if old_nearest != Some(new_nearest) {
            self.set_hw_compare(new_nearest);
        }

        Ok(())
    }

    pub fn cancel(&mut self, id: u8) -> bool {
        match self
            .entries
            .iter_mut()
            .find(|e| e.active && e.event_id == id)
        {
            Some(slot) => {
                slot.active = false;
                self.count -= 1;
                match self.nearest_deadline() {
                    Some(deadline) => self.set_hw_compare(deadline),
                    None => unsafe {
                        ffi::rtc::c_rtc_clear_cc0();
                    },
                }
                true
            }
            None => false,
        }
    }

    fn now(&self) -> u32 {
        unsafe { ffi::rtc::c_rtc_read_current_ticks() }
    }

    fn nearest_deadline(&self) -> Option<u32> {
        self.entries
            .iter()
            // only active
            .filter(|e| e.active)
            // by deadline
            .map(|e| e.deadline)
            // compare deadlines
            .min_by(|a, b| {
                if a == b {
                    Ordering::Equal
                } else if b.wrapping_sub(*a) < 0x8000_0000 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
    }

    fn set_hw_compare(&mut self, deadline: u32) {
        unsafe {
            ffi::rtc::c_rtc_set_cc0(deadline);
        };
    }

    pub fn handle_tick(&mut self, mut upon: impl FnMut(u8)) {
        let now = self.now();

        for entry in &mut self.entries {
            if !entry.active {
                continue;
            }

            if !is_expired(entry.deadline, now) {
                continue;
            }

            upon(entry.event_id);

            match entry.period {
                Some(period) => {
                    // reschedule it & keep active
                    entry.deadline = entry.deadline.wrapping_add(period.get());
                }
                None => {
                    entry.active = false;
                    self.count -= 1;
                }
            }
        }

        // re-arm
        match self.nearest_deadline() {
            Some(next) => self.set_hw_compare(next),
            None => unsafe { ffi::rtc::c_rtc_clear_cc0() },
        }
    }
}

fn is_expired(deadline: u32, now: u32) -> bool {
    now.wrapping_sub(deadline) < 0x8000_0000
}
