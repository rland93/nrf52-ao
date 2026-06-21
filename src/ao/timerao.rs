use crate::ao;
use crate::drivers::timer::TimerScheduler;

#[derive(Clone, Copy)]
pub enum TimerEvent {
    Empty,
    Tick,
}

pub static TIMER_AO: ao::AoContext<TimerSM<16>, ao::P3, 8> = ao::AoContext::new(TimerSM {
    scheduler: TimerScheduler::new(),
});

impl ao::ConstNullable for TimerEvent {
    const NONE: Self = TimerEvent::Empty;
}

pub struct TimerSM<const N: usize> {
    scheduler: TimerScheduler<N>,
}

impl<const N: usize> ao::StateMachine for TimerSM<N> {
    type Event = TimerEvent;

    fn handle_event(&mut self, event: TimerEvent) {
        match event {
            TimerEvent::Empty => {}
            TimerEvent::Tick => self.scheduler.handle_tick(|_event_id| {
                // event id is the id to post to when the timer fires
            }),
        }
    }
}
