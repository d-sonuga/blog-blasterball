#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[no_mangle]
extern "efiapi" fn efi_main(handle: *const core::ffi::c_void, sys_table: *mut SystemTable) -> usize {
    // The array that will hold the UTF-16 characters.
    // The length of "Hello World!\n" is 13, but the array's length is 14 because it must
    // be null-terminated, that is, it must end with a 0.
    let mut string_u16 = [0u16; 14];
    // The string as a string slice
    let string = "Hello World!\n";
    // Converting the string slice to UTF-16 characters and placing the characters
    // in the array
    string.encode_utf16()
        .enumerate()
        .for_each(|(i, letter)| string_u16[i] = letter);
    // Getting the pointer to the Simple Text Output Protocol from the System Table
    let simple_text_output = unsafe { (*sys_table).simple_text_output };
    // Getting the output_string function from the Simple Text Output Protocol and
    // calling it with the required parameters to print "Hello World!\n"
    unsafe { ((*simple_text_output).output_string)(simple_text_output, string_u16.as_mut_ptr()); }
    // Returning 0 because the function expects it
    0
}

// NEW:
#[repr(C)]
struct SystemTable {
    unneeded: [u8; 60],
    simple_text_output: *mut SimpleTextOutput
}

// NEW:
#[repr(C)]
struct SimpleTextOutput {
    unneeded: [u8; 8],
    output_string: extern "efiapi" fn (this: *mut SimpleTextOutput, *mut u16)
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}