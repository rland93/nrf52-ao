use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { crate::ffi::gpio::c_gpio_led_set(true, false, false) };
    loop {
        core::hint::spin_loop();
    }
}
