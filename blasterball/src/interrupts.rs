use core::arch::asm;

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