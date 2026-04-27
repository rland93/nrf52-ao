pub(crate) mod egu {
    unsafe extern "C" {
        pub(crate) fn c_trigger_egu(prio: u8);
    }
}

pub(crate) mod gpio {
    unsafe extern "C" {
        pub(crate) fn c_gpio_led_set(r: bool, g: bool, b: bool);
    }
}

pub(crate) mod isr_mask {
    unsafe extern "C" {
        pub(crate) fn c_enter_critical() -> u32;
        pub(crate) fn c_exit_critical(state: u32);
    }
}
