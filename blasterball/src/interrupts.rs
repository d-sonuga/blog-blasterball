use core::arch::asm;
use crate::gdt::SegmentSelector;

// Tells the processor to stop responding to interrupts
pub fn disable_interrupts() {
    unsafe {
        asm!("cli");
    }
}

// Tells the processor to start responding to interrupts
pub fn enable_interrupts() {
    unsafe {
        asm!("sti");
    }
}

// An entry in the IDT
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Entry {
    // The lower 16 bits of the handler's interrupt service routine
    handler_ptr_low: u16,
    // The segment selector of the code segment
    segment_selector: SegmentSelector,
    // The IDT entry options
    options: Options,
    // The next 16 bits of the interrupt service routine
    handler_ptr_middle: u16,
    // The upper 32 bits of the interrupt service routine
    handler_ptr_upper: u32,
    reserved: u32
}

impl Entry {
    // Creates an empty entry
    fn empty() -> Self {
        Self {
            handler_ptr_low: 0,
            segment_selector: SegmentSelector(0),
            options: Options(0),
            handler_ptr_middle: 0,
            handler_ptr_upper: 0,
            reserved: 0
        }
    }
}

// Options in an IDT entry
#[repr(transparent)]
#[derive(Clone, Copy)]
struct Options(u16);

// The Interrupt Descriptor Table
#[repr(C, align(8))]
pub struct IDT {
    pub divide_by_zero: Entry,
    pub debug: Entry,
    pub nmi_interrupt: Entry,
    pub breakpoint: Entry,
    pub overflow: Entry,
    pub bound_range_exceeded: Entry,
    pub invalid_opcode: Entry,
    pub device_not_available: Entry,
    pub double_fault: Entry,
    pub coprocessor_segment_overrun: Entry,
    pub invalid_tss: Entry,
    pub segment_not_present: Entry,
    pub stack_segment_fault: Entry,
    pub general_protection: Entry,
    pub page_fault: Entry,
    reserved1: Entry,
    pub floating_point_error: Entry,
    pub alignment_check: Entry,
    pub machine_check: Entry,
    pub simd_floating_point_exception: Entry,
    pub virtualization_exception: Entry,
    pub control_protection_exception: Entry,
    reserved2: [Entry; 10],
    pub interrupts: [Entry; 256 - 32]
}

impl IDT {
    pub fn new() -> Self {
        Self {
            divide_by_zero: Entry::empty(),
            debug: Entry::empty(),
            nmi_interrupt: Entry::empty(),
            breakpoint: Entry::empty(),
            overflow: Entry::empty(),
            bound_range_exceeded: Entry::empty(),
            invalid_opcode: Entry::empty(),
            device_not_available: Entry::empty(),
            double_fault: Entry::empty(),
            coprocessor_segment_overrun: Entry::empty(),
            invalid_tss: Entry::empty(),
            segment_not_present: Entry::empty(),
            stack_segment_fault: Entry::empty(),
            general_protection: Entry::empty(),
            page_fault: Entry::empty(),
            reserved1: Entry::empty(),
            floating_point_error: Entry::empty(),
            alignment_check: Entry::empty(),
            machine_check: Entry::empty(),
            simd_floating_point_exception: Entry::empty(),
            virtualization_exception: Entry::empty(),
            control_protection_exception: Entry::empty(),
            reserved2: [Entry::empty(); 10],
            interrupts: [Entry::empty(); 256 - 32]
        }
    }
}

// The values pushed on the stack when an interrupt service routine
// is called by the processor
#[repr(C)]
struct InterruptStackFrame {
    // The address of the instruction that was executing
    // before the processor switched to the service routine
    original_instruction_ptr: u64,
    // The code segment selector that was being used before
    // the control switch to the service routine
    // It is padded to become 64 bits
    original_code_segment: u64,
    // The contents of the FLAGS register at the time of
    // the control switch to the service routine
    flags: u64,
    // Address of the stack at the time of the switch
    original_stack_ptr: u64,
    // The stack segment selector at the time of the switch
    original_stack_segment: u64
}

type ServiceRoutine = extern "x86-interrupt" fn(InterruptStackFrame);

type ServiceRoutineWithErrCode = extern "x86-interrupt" fn(InterruptStackFrame, u64);

type ServiceRoutineWithNoReturn = extern "x86-interrupt" fn(InterruptStackFrame, u64) -> !;