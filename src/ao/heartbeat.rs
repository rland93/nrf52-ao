/// heartbeat.rs
///
use crate::ao::ConstNullable;
use crate::ao::ao::{AoContext, Priority, StateMachine};
use crate::drivers::led::Color;

// all possible events that can be posted to this AO.
#[derive(Clone, Copy)]
pub enum HeartbeatEvent {
    Empty,
    Tick,
}

// any event that can be posted must have a member which represents empty.
impl ConstNullable for HeartbeatEvent {
    const NONE: Self = HeartbeatEvent::Empty;
}

// the main struct holding state etc.
pub struct HeartbeatSM {
    color: Color,
}

impl HeartbeatSM {
    fn next_color(color: Color) -> Color {
        match color {
            Color::Off => Color::Red,
            Color::Red => Color::Blue,
            Color::Blue => Color::Green,
            Color::Green => Color::White,
            Color::White => Color::Off,
        }
    }
}

// the state machine to track this object's change in state in response to events.
impl StateMachine for HeartbeatSM {
    type Event = HeartbeatEvent;

    fn handle_event(&mut self, event: Self::Event) {
        match event {
            HeartbeatEvent::Tick => {
                self.color = Self::next_color(self.color);
                crate::drivers::led::set(self.color);
            }
            HeartbeatEvent::Empty => {
                // TODO: when do we ever get here?
            }
        }
    }
}

// concrete representation
pub static HEARTBEAT_AO: AoContext<HeartbeatSM, 8> =
    AoContext::new(Priority::High, HeartbeatSM { color: Color::Off });

// callable by C code to have ISR's posting to this AO.
#[unsafe(no_mangle)]
pub extern "C" fn rust_post_heartbeat_event() {
    HEARTBEAT_AO.post(HeartbeatEvent::Tick);
}
