use spin::Lazy;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::{gdt, println};

pub mod pic;

pub static INTERUPT_DESCRIPTOR_TABLE: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut interrupt_descriptor_table = InterruptDescriptorTable::new();
    interrupt_descriptor_table
        .breakpoint
        .set_handler_fn(breakpoint_handler);
    interrupt_descriptor_table
        .page_fault
        .set_handler_fn(page_fault_handler);
    unsafe {
        interrupt_descriptor_table
            .double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX)
    };

    pic::set_pic_handlers(&mut interrupt_descriptor_table);

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

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    todo!("Handle loading new page");
    // crate::hlt_loop();
}

#[cfg(test)]
mod tests {
    #[test_case]
    fn test_breakpoint_handler() {
        x86_64::instructions::interrupts::int3();
    }
}
