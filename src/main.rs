#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    print!("Hello Richard!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
