mod ao;
mod evtqueue;
mod heartbeat;
mod pool;
mod timerao;

pub use ao::*;

use heartbeat::HEARTBEAT_AO;

/// type can have a null value when declared as a constant
pub trait ConstNullable {
    const NONE: Self;
}

impl<T> ConstNullable for Option<T> {
    const NONE: Self = None;
}

pub static PRIO_3_AOS: &[&dyn ao::DispatchableAt<ao::P3>] = &[];
pub static PRIO_5_AOS: &[&dyn ao::DispatchableAt<ao::P5>] = &[&HEARTBEAT_AO];
pub static PRIO_6_AOS: &[&dyn ao::DispatchableAt<ao::P6>] = &[];
pub static PRIO_7_AOS: &[&dyn ao::DispatchableAt<ao::P7>] = &[];
