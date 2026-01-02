#![no_std]
#![no_main]

#[macro_use]
#[path = "../common/mod.rs"]
mod common;

use rust_os;

should_panic_test!(stack_overflow);

#[allow(unconditional_recursion)]
fn stack_overflow() {
    core::hint::black_box(stack_overflow());
}
