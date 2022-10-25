#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[no_mangle]
extern "efiapi" fn efi_main(handle: *const core::ffi::c_void, sys_table: *mut u8) -> usize {
    0
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}