use spin::Lazy;
use x86_64::{
    VirtAddr,
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

pub struct GlobalDescriptorTableAccessor {
    pub global_descriptor_table: GlobalDescriptorTable,
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub static TASK_STATE_SEGMENT: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut task_state_segment = TaskStateSegment::new();
    task_state_segment.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        // TODO replace with a stack
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let stack_start = VirtAddr::from_ptr(&raw const STACK);
        let stack_end = stack_start + (STACK_SIZE as u64);
        stack_end
    };
    task_state_segment
});

pub static GLOBAL_DESCRIPTOR_TABLE: Lazy<GlobalDescriptorTableAccessor> = Lazy::new(|| {
    let mut global_descriptor_table = GlobalDescriptorTable::new();

    let code_selector = global_descriptor_table.append(Descriptor::kernel_code_segment());
    let tss_selector =
        global_descriptor_table.append(Descriptor::tss_segment(&*TASK_STATE_SEGMENT));

    GlobalDescriptorTableAccessor {
        global_descriptor_table,
        tss_selector,
        code_selector,
    }
});

pub fn init() {
    use x86_64::instructions::segmentation::{CS, Segment};
    use x86_64::instructions::tables::load_tss;

    GLOBAL_DESCRIPTOR_TABLE.global_descriptor_table.load();
    unsafe {
        CS::set_reg(GLOBAL_DESCRIPTOR_TABLE.code_selector);
        load_tss(GLOBAL_DESCRIPTOR_TABLE.tss_selector);
    }
}
