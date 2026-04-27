#![no_std]

use core::arch::asm;

mod ao;
mod drivers;
mod ffi;
mod panic;

#[unsafe(no_mangle)]
pub extern "C" fn rust_app_main() -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}
