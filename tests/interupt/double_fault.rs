#![no_std]
#![no_main]

#[macro_use]
#[path = "../common/mod.rs"]
mod common;

use rust_os;

should_panic_test!(double_fault);

fn double_fault() {
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
}
