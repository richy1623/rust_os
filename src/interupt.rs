use spin::Lazy;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::{gdt, println};

pub static INTERUPT_DESCRIPTOR_TABLE: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut interrupt_descriptor_table = InterruptDescriptorTable::new();
    interrupt_descriptor_table
        .breakpoint
        .set_handler_fn(breakpoint_handler);
    unsafe {
        interrupt_descriptor_table
            .double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX)
    };

    interrupt_descriptor_table
});

pub fn init() {
    INTERUPT_DESCRIPTOR_TABLE.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_breakpoint_handler() {
        x86_64::instructions::interrupts::int3();
    }
}
