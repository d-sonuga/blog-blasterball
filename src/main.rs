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

#[repr(C)]
struct SystemTable {
    unneeded: [u8; 60],
    simple_text_output: *mut SimpleTextOutput
}

#[repr(C)]
struct SimpleTextOutput {
    unneeded: [u8; 8],
    output_string: extern "efiapi" fn (this: *mut SimpleTextOutput, *mut u16)
}

// The Graphics Output Protocol which has some useful utilities for handling
// drawing to the screen
#[repr(C)]
struct GraphicsOutput {
    // This function collects information about the graphics mode
    // specified in `mode_number` and puts a pointer to that information
    // in the location pointed to by `info`
    // This returns a usize which tells if the function was successful
    query_mode: extern "efiapi" fn(
        // A pointer to the `GraphicsOutput` instance
        this: *mut GraphicsOutput,
        // The number associated with the mode which you
        // want to get information about
        mode_number: u32,
        // The size of the buffer in **info
        size_of_info: *const usize,
        // The pointer to a location in which the firmware will place a pointer
        // to the information collected on a successful return
        info: *const *const GraphicsModeInfo
    ) -> usize,
}

// The blueprint to intepret the bits in **info upon a successful return from calling the
// GraphicsOutput's `query_mode` function
#[repr(C)]
struct GraphicsModeInfo {
    // The UEFI version number of this data structure
    version: u32,
    // The number of pixels that can be contained in one
    // horizontal row of the video screen in the mode whose info was requested
    horizontal_resolution: u32,
    // The number of pixels that can be contained in one vertical
    // column of the video screen in this mode whose info was requested
    vertical_resolution: u32,
    // Indicates how the bits of representing a single pixel should
    // be interpreted
    pixel_format: PixelFormat,
    // Some value whose meaning depends on the value of `pixel_format`
    pixel_info: PixelBitmask,
    // The number of pixels in one line of video memory.
    // Similar to `horizontal_resolution`, but different in a few way I think
    // are irrelevant
    pixels_per_scan_line: u32
}

// Defines how to interpret the bits that represent a single pixel
#[repr(transparent)]
struct PixelFormat(u32);

impl PixelFormat {
    const RED_GREEN_BLUE_RESERVED: u32 = 0;
    const BLUE_GREEN_RED_RESERVED: u32 = 1;
    const BIT_MASK: u32 = 2;
    const BLT_ONLY: u32 = 3;
    const FORMAT_MAX: u32 = 4;
}

// A description of the color channels of a pixel in the GOP's framebuffer
#[repr(C)]
struct Pixel {
    // The bits representing the blue color intensity in this pixel
    blue: u8,
    // The bits representing the green color intensity in this pixel
    green: u8,
    // The bits representing the red color intensity in this pixel
    red: u8,
    // Unused bits
    reserved: u8
}

// A structure telling how to re-interpret the bits in a pixel instance
// when the `GraphicsModeInfo` instance is set to `PixelFormat::BIT_MASK`
#[repr(C)]
struct PixelBitmask {
    // The bits set to 1 in this field tells which bits in a pixel should be
    // interpreted as the red color intensity when the `GraphicsModeInfo` instance
    // is set to `PixelFormat::BIT_MASK`
    red_mask: u32,
    // The bits set to 1 in this field tells which bits in a pixel should be
    // interpreted as the green color intensity when the `GraphicsModeInfo` instance
    // is set to `PixelFormat::BIT_MASK`
    green_mask: u32,
    // The bits set to 1 in this field tells which bits in a pixel should be
    // interpreted as the blue color intensity when the `GraphicsModeInfo` instance
    // is set to `PixelFormat::BIT_MASK`
    blue_mask: u32,
    // The bits set to 1 in this field tells which bits in a pixel should be
    // interpreted as the reserved field when the `GraphicsModeInfo` instance
    // is set to `PixelFormat::BIT_MASK`
    reserved: u32
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}