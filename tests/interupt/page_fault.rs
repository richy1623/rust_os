#![no_std]
#![no_main]

#[macro_use]
#[path = "../common/mod.rs"]
mod common;

use rust_os;

should_run_test!(page_fault);

fn page_fault() {
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
}
