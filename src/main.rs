#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[no_mangle]
extern "efiapi" fn efi_main(handle: *const core::ffi::c_void, sys_table: *mut SystemTable) -> usize {
    // Getting the pointer to the Boot Services from the System Table
    let boot_services = unsafe { (*sys_table).boot_services };
    // The Graphics Output Protocol (GOP) GUID
    let gop_guid = Guid {
        first_chunk: 0x9042a9de,
        second_chunk: 0x23dc,
        third_chunk: 0x4a38,
        other_chunks: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a]
    };
    // The location which will hold a pointer to a GOP on a successful call to locate_protocol
    let mut gop: *mut core::ffi::c_void = core::ptr::null_mut();
    // The raw pointer to the GOP Guid
    let guid_ptr = &gop_guid as *const Guid;
    // An optional argument which we're just going to pass null into
    let registration = core::ptr::null_mut();
    // Location where the GOP pointer should be placed into on a successful locate_protocol invocation
    let gop_ptr = &mut gop as *mut _;
    // Invoking the Boot Services locate_protocol function to find the GOP
    let locate_gop_status = unsafe { ((*boot_services).locate_protocol)(
        guid_ptr,
        registration,
        gop_ptr
    ) };

    if locate_gop_status != 0 {
        let mut string_u16 = [0u16; 21];
        // The string as a string slice
        let string = "Failed to locate GOP\n";
        // Converting the string slice to UTF-16 characters and placing the characters
        // in the array
        string.encode_utf16()
            .enumerate()
            .for_each(|(i, letter)| string_u16[i] = letter);
        // Getting the pointer to the Simple Text Output Protocol from the System Table
        let simple_text_output = unsafe { (*sys_table).simple_text_output };
        // Getting the output_string function from the Simple Text Output Protocol and
        // calling it with the required parameters to print "Failed to locate GOP\n"
        unsafe { ((*simple_text_output).output_string)(simple_text_output, string_u16.as_mut_ptr()); }
        loop {}
    }

    // At this point, it is safe to dereference the `gop` because the locate protocol
    // executed successfully, as verified from the `locate_gop_status`
    // We first cast the gop as a pointer to the GraphicsOutput
    let gop = gop as *mut GraphicsOutput;
    // Get the mode pointer from the GOP instance
    let mode = unsafe { (*gop).mode };
    // Get the value of the max mode number from the `mode`
    let max_mode = unsafe { (*mode).max_mode };
    // The valid mode numbers are in the range 0..=`max_mode`-1
    for mode_number in 0..max_mode {
        // The size of our `GraphicsModeInfo` structure
        let size_of_info = core::mem::size_of::<GraphicsModeInfo>();
        // The location that will hold the pointer to the `GraphicsModeInfo`
        // for the current `mode_number` on a successful call to the
        // `GraphicsOutput` query_mode`
        let mut mode: *const GraphicsModeInfo = core::ptr::null_mut();
        // Gettings the `query_mode` function from the GOP instance
        let query_mode = unsafe { (*gop).query_mode };
        // Calling `query_mode` to get information about the mode associated
        // with `mode_number`
        let query_status = (query_mode)(
            // The pointer to the GOP instance
            gop,
            // The mode number associated with the mode we want information about
            mode_number,
            // The size of the `GraphicsModeInfo` structure
            &size_of_info as *const _,
            // The pointer to the location to be mutated to hold the pointer to the
            // `GraphicsModeInfo` instance associated with the current `mode_number`
            // on a successful function execution
            &mut mode as *mut _
        );
        // Checking if the status is not a success
        // If it's not print and error message and halt (loop endlessly)
        if query_status != 0 {
            let mut string_u16 = [0u16; 18];
            let string = "query_mode failed\n";
            string.encode_utf16()
                .enumerate()
                .for_each(|(i, letter)| string_u16[i] = letter);
            let simple_text_output = unsafe { (*sys_table).simple_text_output };
            unsafe { ((*simple_text_output).output_string)(simple_text_output, string_u16.as_mut_ptr()); }
            loop {}
        }
    }

    // Printing a success message after querying all the modes successfully
    let mut string_u16 = [0u16; 31];
    let string = "Successfully queried all modes\n";
    string.encode_utf16()
        .enumerate()
        .for_each(|(i, letter)| string_u16[i] = letter);
    let simple_text_output = unsafe { (*sys_table).simple_text_output };
    unsafe { ((*simple_text_output).output_string)(simple_text_output, string_u16.as_mut_ptr()); }

    // Returning 0 because the function expects it
    0
}

#[repr(C)]
struct SystemTable {
    unneeded: [u8; 60],
    simple_text_output: *mut SimpleTextOutput,
    unneeded2: [u8; 24],
    boot_services: *const BootServices
}

// A number that uniquely identifies a protocol
#[repr(C)]
struct Guid {
    first_chunk: u32,
    second_chunk: u16,
    third_chunk: u16,
    other_chunks: [u8; 8]
}

// Returned by the UEFI functions to indicate the success
// or failure of the function
type Status = usize;

// A bunch of other useful functions provided by the firmware
// and accessible from the `SystemTable`
#[repr(C)]
struct BootServices {
    // We don't need these other fields
    unneeded: [u8; 320],
    // A function that can be used to find a protocol by its
    // unique GUID
    locate_protocol: extern "efiapi" fn(
        protocol: *const Guid,
        registration: *const core::ffi::c_void,
        interface: *mut *mut core::ffi::c_void
    ) -> Status
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
        info: *mut *const GraphicsModeInfo
    ) -> Status,
    // Sets the video device into the mode associated with `mode_number` and clears
    // the visible portions of the output display to black
    set_mode: extern "efiapi" fn(
        this: *mut GraphicsOutput,
        mode_number: u32
    ) -> Status,
    // The Blt function pointer, which we don't need
    unneeded: [u8; 8],
    // Gives information about the current graphics mode
    // and the other available modes
    mode: *const GraphicsMode
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
#[repr(u32)]
enum PixelFormat {
    RedGreenBlueReserved = 0,
    BlueGreenRedReserved = 1,
    BitMask = 2,
    BltOnly = 3
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

// The physical address on x86_64 is 8 bytes (64 bits)
type PhysAddr = u64;

// Gives info about the currently set and other available graphics modes
#[repr(C)]
struct GraphicsMode {
    // The number of modes supported by `GraphicsOutput::set_mode`
    // and `GraphicsOutput::query_mode`
    max_mode: u32,
    // The number associated with the current mode of the graphics
    // device. Valid values are always in the range 0..=`max_mode`-1
    mode: u32,
    // Pointer to a read only GraphicsModeInfo
    info: *const GraphicsModeInfo,
    // Size of the `GraphicsModeInfo` structure
    size_of_info: usize,
    // The starting address of the graphics framebuffer
    framebuffer_base: PhysAddr,
    // The size of the framebuffer in bytes
    framebuffer_size: usize
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    loop {}
}