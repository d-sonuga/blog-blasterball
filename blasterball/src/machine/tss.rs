use core::arch::asm;
use crate::gdt::SegmentSelector;

// The Task State Segment
#[repr(C, packed)]
pub struct TSS {
    reserved1: u32,
    // Stack pointers for different privilege levels
    privilege_stack_table: [*mut u8; 3],
    reserved2: u64,
    // Stack pointers for switching stacks when an interrupt handler wants to
    interrupt_stack_table: [*mut u8; 7],
    reserved3: u64,
    reserved4: u16,
    // Offset from the TSS start address to the IO permission bit map
    io_map_base_addr: u16
}

unsafe impl Sync for TSS {}

impl TSS {
    // Creates a new TSS with all the stack pointers set to null pointers
    // and no IO permission bit map
    pub const fn new() -> Self {
        Self {
            privilege_stack_table: [core::ptr::null_mut(); 3],
            interrupt_stack_table: [core::ptr::null_mut(); 7],
            // Setting this field to the size of the TSS indicates that
            // there is no IO permission bit map
            io_map_base_addr: core::mem::size_of::<TSS>() as u16,
            // Always better to leave the reserved fields as 0s
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
            reserved4: 0
        }
    }
}

pub fn load_tss(sel: SegmentSelector) {
    unsafe {
        asm!("ltr {0:x}", in(reg) sel.0);
    }
}
