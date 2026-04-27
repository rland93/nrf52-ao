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

pub(crate) mod timer {
    unsafe extern "C" {
        // get current ticks for a channel
        pub(crate) fn c_timer3_current_ticks(channel: u8);

        // set compare value for a channel; trigger interrupt when timer reaches this value.
        pub(crate) fn c_timer3_set(channel: u8, ticks: u32);

        // clear the interrupt for a channel
        pub(crate) fn c_timer3_clear(channel: u8);

        // get the value for a channel
        pub(crate) fn c_timer3_capture(channel: u8) -> u32;

        // enable the interrupt for a channel
        pub(crate) fn c_timer3_enable_compare_int(channel: u8);

        // start the timer
        pub(crate) fn c_timer3_start();

        // stop the timer
        pub(crate) fn c_timer3_stop();
    }
}
