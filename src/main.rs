#![no_std]
#![no_main]

#[cfg(not(test))]
use core::panic::PanicInfo;
#[cfg(not(test))]
use rust_os::*;

#[cfg(not(test))]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    rust_os::init();
    print!("Hello Richard!");
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("\n{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
