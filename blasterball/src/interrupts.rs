use core::arch::asm;
use crate::gdt::SegmentSelector;
use core::marker::PhantomData;
use core::ops::{BitAnd, Shr};

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
pub struct Entry<T> {
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
    reserved: u32,
    // Dummy field to resolve problem of no field making
    // use of generic type
    phantom: PhantomData<T>
}

impl<T: BitAnd<u16, Output=u16> 
    + Shr<u64, Output=u16> 
    + Shr<u32, Output=u32> 
    + Copy> Entry<T> {
    // Creates an empty entry
    fn empty() -> Self {
        Self {
            handler_ptr_low: 0,
            segment_selector: SegmentSelector(0),
            options: Options(0),
            handler_ptr_middle: 0,
            handler_ptr_upper: 0,
            reserved: 0,
            phantom: PhantomData
        }
    }

    // Creates a new entry for exceptions
    pub fn exception(handler_ptr: T, segment_selector: SegmentSelector) -> Self {
        Self::new(handler_ptr, segment_selector, Options::exception())
    }

    // Creates a new entry for interrupts
    pub fn interrupt(handler_ptr: T, segment_selector: SegmentSelector) -> Self {
        Self::new(handler_ptr, segment_selector, Options::interrupt())
    }

    // Creates a new entry with the specified options
    fn new(handler_ptr: T, segment_selector: SegmentSelector, options: Options) -> Self {
        let mut entry = Self::empty();
        // Clearing out all bits except the lower 16 bits of the routine
        // function pointer and setting it as the entry's handler pointer lower bits
        entry.handler_ptr_low = handler_ptr & 0xffffu16;
        // Shifting out the lower 16 bits leaving the next 16 bits in its
        // place and setting it as the entry's handler pointer middle bits
        entry.handler_ptr_middle = (handler_ptr >> 16u64) & 0xffffu16;
        // Clearing out the lower 32 bits and retaining only the upper 32
        // Then setting it as the entry's handler pointer higher bits
        entry.handler_ptr_upper = (handler_ptr >> 32u32) & 0xffffffffu32;

        // Setting the segment selector of the entry
        entry.segment_selector = segment_selector;

        entry.options = options;

        entry
    }
}

// Options in an IDT entry
#[repr(transparent)]
struct Options(u16);

impl Options {
    // Creates the options for an exception entry in the IDT
    fn exception() -> Self {
        Self(0b1000111100000000)
    }

    // Creates the options for a maskable interrupt entry in the IDT
    fn interrupt() -> Self {
        Self(0b1000111000000000)
    }
}

// The Interrupt Descriptor Table
#[repr(C, align(8))]
pub struct IDT {
    pub divide_by_zero: Entry<ServiceRoutine>,
    pub debug: Entry<ServiceRoutine>,
    pub nmi_interrupt: Entry<ServiceRoutine>,
    pub breakpoint: Entry<ServiceRoutine>,
    pub overflow: Entry<ServiceRoutine>,
    pub bound_range_exceeded: Entry<ServiceRoutine>,
    pub invalid_opcode: Entry<ServiceRoutine>,
    pub device_not_available: Entry<ServiceRoutine>,
    pub double_fault: Entry<ServiceRoutineWithNoReturn>,
    pub coprocessor_segment_overrun: Entry<ServiceRoutine>,
    pub invalid_tss: Entry<ServiceRoutineWithErrCode>,
    pub segment_not_present: Entry<ServiceRoutineWithErrCode>,
    pub stack_segment_fault: Entry<ServiceRoutineWithErrCode>,
    pub general_protection: Entry<ServiceRoutineWithErrCode>,
    pub page_fault: Entry<ServiceRoutineWithErrCode>,
    reserved1: Entry<ServiceRoutine>,
    pub floating_point_error: Entry<ServiceRoutine>,
    pub alignment_check: Entry<ServiceRoutineWithErrCode>,
    pub machine_check: Entry<ServiceRoutineWithNoReturn>,
    pub simd_floating_point_exception: Entry<ServiceRoutine>,
    pub virtualization_exception: Entry<ServiceRoutine>,
    pub control_protection_exception: Entry<ServiceRoutineWithErrCode>,
    reserved2: [Entry<ServiceRoutine>; 10],
    pub interrupts: [Entry<ServiceRoutine>; 256 - 32]
}

// The values pushed on the stack when an interrupt service routine
// is called by the processor
#[repr(C)]
pub struct InterruptStackFrame {
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

#[derive(Clone, Copy)]
pub struct ServiceRoutine(pub extern "x86-interrupt" fn(InterruptStackFrame));

#[derive(Clone, Copy)]
pub struct ServiceRoutineWithErrCode(pub extern "x86-interrupt" fn(InterruptStackFrame, u64));

#[derive(Clone, Copy)]
pub struct ServiceRoutineWithNoReturn(pub extern "x86-interrupt" fn(InterruptStackFrame, u64) -> !);

impl BitAnd<u16> for ServiceRoutine {
    type Output = u16;

    fn bitand(self, rhs: u16) -> u16 {
        (self.0 as u16) & rhs
    }
}

impl BitAnd<u16> for ServiceRoutineWithErrCode {
    type Output = u16;

    fn bitand(self, rhs: u16) -> u16 {
        (self.0 as u16) & rhs
    }
}

impl BitAnd<u16> for ServiceRoutineWithNoReturn {
    type Output = u16;

    fn bitand(self, rhs: u16) -> u16 {
        (self.0 as u16) & rhs
    }
}

impl Shr<u64> for ServiceRoutine {
    type Output = u16;

    fn shr(self, rhs: u64) -> u16 {
        (self.0 as u64 >> rhs) as u16
    }
}

impl Shr<u64> for ServiceRoutineWithErrCode {
    type Output = u16;

    fn shr(self, rhs: u64) -> u16 {
        (self.0 as u64 >> rhs) as u16
    }
}

impl Shr<u64> for ServiceRoutineWithNoReturn {
    type Output = u16;

    fn shr(self, rhs: u64) -> u16 {
        (self.0 as u64 >> rhs) as u16
    }
}

impl Shr<u32> for ServiceRoutine {
    type Output = u32;

    fn shr(self, rhs: u32) -> u32 {
        (self.0 as u64 >> rhs as u64) as u32
    }
}

impl Shr<u32> for ServiceRoutineWithErrCode {
    type Output = u32;

    fn shr(self, rhs: u32) -> u32 {
        (self.0 as u64 >> rhs as u64) as u32
    }
}

impl Shr<u32> for ServiceRoutineWithNoReturn {
    type Output = u32;

    fn shr(self, rhs: u32) -> u32 {
        (self.0 as u64 >> rhs as u64) as u32
    }
}