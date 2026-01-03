use pic8259::ChainedPics;
use spin::{Lazy, Mutex};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::print;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET + 0,
}

pub fn set_pic_handlers(interrupt_descriptor_table: &mut InterruptDescriptorTable) {
    interrupt_descriptor_table[InterruptIndex::Timer as u8].set_handler_fn(timer_interrupt_handler);
}

pub static PROGRAMMABLE_INTERRUPT_CONTROLLER: Lazy<Mutex<ChainedPics>> =
    Lazy::new(|| Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }));

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");

    unsafe {
        PROGRAMMABLE_INTERRUPT_CONTROLLER
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}
