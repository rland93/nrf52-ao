use crate::ffi;

#[derive(Clone, Copy)]
pub enum Color {
    Off,
    Red,
    Green,
    Blue,
    White,
}

pub fn set(c: Color) {
    let (r, g, b) = match c {
        Color::Off => (false, false, false),
        Color::Red => (true, false, false),
        Color::Green => (false, true, false),
        Color::Blue => (false, false, true),
        Color::White => (true, true, true),
    };
    unsafe { ffi::gpio::c_gpio_led_set(r, g, b) }
}
