mod ao;
mod evtqueue;
mod heartbeat;
mod objects;
mod pool;

/// type can have a null value when declared as a constant
pub trait ConstNullable {
    const NONE: Self;
}

impl<T> ConstNullable for Option<T> {
    const NONE: Self = None;
}
