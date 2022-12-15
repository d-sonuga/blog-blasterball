#![no_std]
#![no_main]
#![feature(abi_efiapi)]

mod font;
use font::FONT;

mod uefi;
use uefi::{SystemTable, Screen, PixelFormat, pre_graphics_print_str, print_str, printint};

mod bitmap;
use bitmap::{FileHeader, DIBHeader, ColorTable, Color};

#[no_mangle]
extern "efiapi" fn efi_main(
    handle: *const core::ffi::c_void,
    sys_table: *mut SystemTable,
) -> usize {
    // Using a reference instead of a raw pointer
    let sys_table = unsafe { &*sys_table };
    // Getting a reference to the Boot Services from the System Table
    let boot_services = sys_table.boot_services();
    // Locate the Graphics Output Protocol
    let gop = boot_services.locate_gop();
    // If location failed, print error and halt
    if gop.is_err() {
        let simple_text_output = sys_table.simple_text_output();
        pre_graphics_print_str(simple_text_output, "Failed to locate GOP\n");
        loop {}
    }
    // Extract the GOP from the result
    let gop = gop.unwrap();
    // Get the mode pointer from the GOP instance
    let mode = gop.mode();
    // Get the value of the max mode number from the `mode`
    let max_mode = mode.max_mode;
    // The desired mode to set after finding it in this for loop
    let mut desired_mode = 0;
    // The valid mode numbers are in the range 0..=`max_mode`-1
    for mode_number in 0..max_mode {
        // Get the mode info
        let mode = gop.query_mode(mode_number);
        if mode.is_err() {
            let simple_text_output = sys_table.simple_text_output();
            pre_graphics_print_str(simple_text_output, "Failed to locate GOP\n");
            loop {}
        }
        // Extract the mode info from the result
        let mode = mode.unwrap();

        let horizontal_resolution = mode.horizontal_resolution;
        let vertical_resolution = mode.vertical_resolution;
        let pixel_format = mode.pixel_format;
        // Looking for our desired mode
        if horizontal_resolution == 640
            && vertical_resolution == 480
            && pixel_format == PixelFormat::BlueGreenRedReserved
        {
            desired_mode = mode_number;
            break;
        }
        // Halt with an error if the desired mode wasn't found
        if mode_number == max_mode - 1 {
            let simple_text_output = sys_table.simple_text_output();
            pre_graphics_print_str(simple_text_output, "Failed to locate GOP\n");
            loop {}
        }
    }

    // Setting the mode to our desired mode
    let set_mode_result = gop.set_mode(desired_mode);

    // Checking if it is a success
    if set_mode_result.is_err() {
        let simple_text_output = sys_table.simple_text_output();
        pre_graphics_print_str(simple_text_output, "Failed to set the desired mode\n");
        loop {}
    }

    // Setting the address of the pixel memory to framebuffer_base
    let framebuffer_base = mode.framebuffer_base;

    // Interpreting the address of the pixel memory as an instance of the Screen struct
    let screen = framebuffer_base as *mut Screen;
    
    // Obtain a mutable reference from the screen's raw pointer
    let screen = unsafe { &mut *screen };

    // Throw block.bmp's bits into the output binary
    // Retrieve a slice to those bits
    let block_bytes = include_bytes!("./block.bmp");

    // Retrieving a pointer to the block.bmp's bytes
    let block_bytes_ptr: *const u8 = block_bytes.as_ptr();
    // Reinterpreting a pointer to bytes as a pointer to a FileHeader instance
    let file_header_ptr = block_bytes_ptr as *const FileHeader;
    // Interpreting the first section of the bitmap as the file header
    let file_header = unsafe { &(*file_header_ptr) };
    // The number of bytes that make up the FileHeader
    const FILE_HEADER_SIZE: usize = core::mem::size_of::<FileHeader>();
    // The DIB header comes immediately after the file header
    const DIB_HEADER_OFFSET: isize = FILE_HEADER_SIZE as isize;
    // Reinterpreting a pointer to the bytes at offset DIB_HEADER_OFFSET
    // as a pointer to the DIB header
    let dib_header_ptr = unsafe { block_bytes_ptr.offset(DIB_HEADER_OFFSET) as *const DIBHeader };
    // Interpreting the second section of the bitmap as the DIB header
    let dib_header = unsafe { &(*dib_header_ptr) };
    // The number of bytes that make up the DIB header
    const DIB_HEADER_SIZE: usize = core::mem::size_of::<DIBHeader>();
    // The color table comes immediately after the file header and the DIB header
    const COLOR_TABLE_OFFSET: isize = (FILE_HEADER_SIZE + DIB_HEADER_SIZE) as isize;
    // Reinterpreting a pointer to the bytes at offset COLOR_TABLE_OFFSET as a pointer
    // to the color table
    let color_table_ptr = unsafe { block_bytes_ptr.offset(COLOR_TABLE_OFFSET) as *const ColorTable };
    // Interpreting the bytes at `COLOR_TABLE_OFFSET` as the color table
    let color_table = unsafe { &(*color_table_ptr) };

    if dib_header.image_width == 36 && dib_header.image_height == 16 {
        print_str(screen, "Width and height as expected");
    } else {
        print_str(screen, "Something wrong somewhere");
    }

    // Returning 0 because the function expects it
    0
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
