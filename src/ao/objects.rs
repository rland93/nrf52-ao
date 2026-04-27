use crate::ao::ao::Dispatchable;

use crate::ao::heartbeat::HEARTBEAT_AO;

pub static PRIO_3_AOS: &[&dyn Dispatchable] = &[];
pub static PRIO_5_AOS: &[&dyn Dispatchable] = &[&HEARTBEAT_AO];
pub static PRIO_6_AOS: &[&dyn Dispatchable] = &[];
pub static PRIO_7_AOS: &[&dyn Dispatchable] = &[];
