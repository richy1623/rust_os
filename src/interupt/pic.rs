use pc_keyboard::Keyboard;
use pic8259::ChainedPics;
use spin::{Lazy, Mutex};
use x86_64::{
    instructions::port::Port,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::print;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

const DATA_PORT_ADDRESS: u16 = 0x60;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

pub fn set_pic_handlers(interrupt_descriptor_table: &mut InterruptDescriptorTable) {
    interrupt_descriptor_table[InterruptIndex::Timer as u8].set_handler_fn(timer_interrupt_handler);
    interrupt_descriptor_table[InterruptIndex::Keyboard as u8]
        .set_handler_fn(keyboard_interrupt_handler);
}

pub static PROGRAMMABLE_INTERRUPT_CONTROLLER: Lazy<Mutex<ChainedPics>> =
    Lazy::new(|| Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) }));

static DATA_PORT: Lazy<Mutex<Port<u8>>> = Lazy::new(|| Mutex::new(Port::new(DATA_PORT_ADDRESS)));

pub static KEYBOARD: Lazy<
    Mutex<Keyboard<pc_keyboard::layouts::Us104Key, pc_keyboard::ScancodeSet1>>,
> = Lazy::new(|| {
    Mutex::new(Keyboard::new(
        pc_keyboard::ScancodeSet1::new(),
        pc_keyboard::layouts::Us104Key,
        pc_keyboard::HandleControl::Ignore,
    ))
});

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PROGRAMMABLE_INTERRUPT_CONTROLLER
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let scan_code: u8 = unsafe { DATA_PORT.lock().read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scan_code) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                pc_keyboard::DecodedKey::Unicode(character) => print!("{}", character),
                pc_keyboard::DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
    unsafe {
        PROGRAMMABLE_INTERRUPT_CONTROLLER
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8);
    }
}
