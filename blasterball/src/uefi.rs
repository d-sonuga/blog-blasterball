use crate::FONT;

// The status code that UEFI defines as a success
pub const STATUS_SUCCESS: Status = 0;

// The Graphics Output Protocol (GOP) GUID
pub const GOP_GUID: Guid = Guid {
    first_chunk: 0x9042a9de,
    second_chunk: 0x23dc,
    third_chunk: 0x4a38,
    other_chunks: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

// Prints a single character before the switch to graphics mode
pub fn pre_graphics_print_char(simple_text_output: &SimpleTextOutput, c: char) {
    // The UTF-16 encoded string which contains the single character to be printed
    let mut utf16_encoded_buffer = [c as u16, 0];
    (simple_text_output.output_string)(simple_text_output, utf16_encoded_buffer.as_mut_ptr());
}

// Prints a string slice before the switch to a graphics mode
pub fn pre_graphics_print_str(simple_text_output: &SimpleTextOutput, s: &str) {
    for c in s.chars() {
        pre_graphics_print_char(simple_text_output, c);
    }
}

// Prints the string s on screen. s may only consist of big and small
// English characters and "!" and " "
pub fn print_str(screen: &mut Screen, s: &str) {
    // The initial screen position (row, column)
    let mut screen_pos = (0, 0);
    // Iterating over the character's code points
    // We don't need to worry about them being bigger than
    // u8::MAX because the only characters we're considering are 'A'..='Z', 'a'..='z' and
    // "!" and " "
    for c in s.as_bytes() {
        print_char(screen, &FONT[char_to_font_index(*c)], screen_pos);
        // Advance the screen position to the next position on the row
        screen_pos.1 += 16;
        // If there is no more space on the row
        if screen_pos.1 >= NO_OF_PIXELS_IN_A_ROW {
            // Advance to the next row
            screen_pos.0 += 16;
            // Start from the first space on this new row
            screen_pos.1 = 0;
        }
        // If there are no more rows, stop looping
        if screen_pos.0 >= NO_OF_PIXELS_IN_A_COLUMN {
            break;
        }
    }
}

// Print the character described by font_description to the screen at position curr_screen_pos
pub fn print_char(
    screen: &mut Screen,
    font_description: &[[bool; 16]; 16],
    curr_screen_pos: (usize, usize),
) {
    for i in 0..16 {
        for j in 0..16 {
            if font_description[i][j] {
                // Red and green is yellow (which we're using as our foreground color here)
                screen.pixels[curr_screen_pos.0 + i][curr_screen_pos.1 + j] = Pixel {
                    red: 255,
                    green: 255,
                    blue: 0,
                    reserved: 0,
                };
            } else {
                // All 0s is black (which we're using as our background here)
                screen.pixels[curr_screen_pos.0 + i][curr_screen_pos.1 + j] = Pixel {
                    red: 0,
                    green: 0,
                    blue: 0,
                    reserved: 0,
                };
            }
        }
    }
}

// Takes the Unicode code point of a character in 'A'..='Z' or 'a'..='z' or "!" or  " "
// and returns its index into the FONT array
fn char_to_font_index(c: u8) -> usize {
    if c >= 97 {
        // Small letters to big letters
        char_to_font_index(c - 32)
    } else if c == 32 {
        // Space
        26
    } else if c == 33 {
        // Exclamaion mark
        27
    } else {
        // FONT array index for big letters
        (c - 65) as usize
    }
}

// The horizontal resolution of our desired mode
pub const NO_OF_PIXELS_IN_A_ROW: usize = 640;
// The vertical resolution of our desired mode
pub const NO_OF_PIXELS_IN_A_COLUMN: usize = 480;

// The pixels on the screen
pub struct Screen {
    pub pixels: [[Pixel; NO_OF_PIXELS_IN_A_ROW]; NO_OF_PIXELS_IN_A_COLUMN],
}

// A function that prints a single digit in the range 0..=9
pub fn printdigit(n: u32, simple_text_output: &SimpleTextOutput) {
    // The code for the digit that will be printed
    let mut digit_u16 = [48 + n as u16, 0];
    // Printing the digit with the Simple Text Output protocol's output_string function
    (simple_text_output.output_string)(simple_text_output, digit_u16.as_mut_ptr());
}

// Prints an integer n
pub fn printint(n: u32, simple_text_output: &SimpleTextOutput) {
    if n >= 10 {
        let quotient = n / 10;
        let remainder = n % 10;
        printint(quotient, simple_text_output);
        printdigit(remainder, simple_text_output);
    } else {
        printdigit(n, simple_text_output);
    }
}

#[repr(C)]
pub struct SystemTable {
    unneeded: [u8; 60],
    simple_text_output: *mut SimpleTextOutput,
    unneeded2: [u8; 24],
    boot_services: *const BootServices,
}

impl SystemTable {
    // Returns a reference to the Boot Services instance in the System Table
    pub fn boot_services(&self) -> &BootServices {
        unsafe { &*self.boot_services }
    }

    // Returns a reference to the Simple Text Output instance in the System Table
    pub fn simple_text_output(&self) -> &SimpleTextOutput {
        unsafe { &*self.simple_text_output }
    }
}

// A number that uniquely identifies a protocol
#[repr(C)]
pub struct Guid {
    first_chunk: u32,
    second_chunk: u16,
    third_chunk: u16,
    other_chunks: [u8; 8],
}

// Returned by the UEFI functions to indicate the success
// or failure of the function
pub type Status = usize;

// A bunch of other useful functions provided by the firmware
// and accessible from the `SystemTable`
#[repr(C)]
pub struct BootServices {
    // We don't need these other fields
    unneeded: [u8; 320],
    // A function that can be used to find a protocol by its
    // unique GUID
    locate_protocol: extern "efiapi" fn(
        protocol: *const Guid,
        registration: *const core::ffi::c_void,
        interface: *mut *mut core::ffi::c_void,
    ) -> Status,
}

impl BootServices {
    // Finds a protocol with it's unique GUID
    pub fn locate_protocol(&self, protocol_guid: &Guid) -> Result<*mut core::ffi::c_void, usize> {
        // The location which will hold a pointer to a protocol on a successful call to locate_protocol
        let mut protocol: *mut core::ffi::c_void = core::ptr::null_mut();
        // The raw pointer to the protocol Guid
        let guid_ptr = protocol_guid as *const Guid;
        // An optional argument which we're just going to pass null into
        let registration = core::ptr::null_mut();
        // Location where the protocol pointer should be placed into on a successful locate_protocol invocation
        let protocol_ptr = &mut protocol as *mut _;
        // Invoking the Boot Services locate_protocol function to find the protocol
        let locate_protocol_status = (self.locate_protocol)(guid_ptr, registration, protocol_ptr);

        if locate_protocol_status != STATUS_SUCCESS {
            // If the attempt failed, return the failed error code
            Err(locate_protocol_status)
        } else {
            // If the attempt didn't fail, return the pointer to the protocol
            Ok(protocol)
        }
    }

    // Convenience function to locate the Graphics Output Protocol
    pub fn locate_gop(&self) -> Result<&GraphicsOutput, Status> {
        // Attempt to locate the GOP
        let locate_gop = self.locate_protocol(&GOP_GUID);
        if locate_gop.is_ok() {
            // Return a reference to the GOP, instead of a raw pointer
            let gop_ptr = locate_gop.unwrap() as *mut GraphicsOutput;
            let gop = unsafe { &*gop_ptr };
            Ok(gop)
        } else {
            // Return the error code it failed with
            Err(locate_gop.unwrap_err())
        }
    }
}

#[repr(C)]
pub struct SimpleTextOutput {
    unneeded: [u8; 8],
    output_string: extern "efiapi" fn(this: &SimpleTextOutput, *mut u16)
}

// The Graphics Output Protocol which has some useful utilities for handling
// drawing to the screen
#[repr(C)]
pub struct GraphicsOutput {
    // This function collects information about the graphics mode
    // specified in `mode_number` and puts a pointer to that information
    // in the location pointed to by `info`
    // This returns a usize which tells if the function was successful
    query_mode: extern "efiapi" fn(
        // A reference to the `GraphicsOutput` instance
        this: &GraphicsOutput,
        // The number associated with the mode which you
        // want to get information about
        mode_number: u32,
        // The size of the buffer in **info
        size_of_info: *const usize,
        // The pointer to a location in which the firmware will place a pointer
        // to the information collected on a successful return
        info: *mut *const GraphicsModeInfo,
    ) -> Status,
    // Sets the video device into the mode associated with `mode_number` and clears
    // the visible portions of the output display to black
    set_mode: extern "efiapi" fn(this: &GraphicsOutput, mode_number: u32) -> Status,
    // The Blt function pointer, which we don't need
    unneeded: [u8; 8],
    // Gives information about the current graphics mode
    // and the other available modes
    mode: *const GraphicsMode,
}

impl GraphicsOutput {
    // Retrieves a reference to the GraphicsMode
    pub fn mode(&self) -> &GraphicsMode {
        unsafe { &*self.mode }
    }

    // Gets information about a mode
    pub fn query_mode(&self, mode_number: u32) -> Result<&GraphicsModeInfo, Status> {
        // The size of our `GraphicsModeInfo` structure
        let size_of_info = core::mem::size_of::<GraphicsModeInfo>();
        // The location that will hold the pointer to the `GraphicsModeInfo`
        // for the current `mode_number` on a successful call to the
        // `GraphicsOutput` query_mode`
        let mut mode: *const GraphicsModeInfo = core::ptr::null_mut();
        // Calling `query_mode` to get information about the mode associated
        // with `mode_number`
        let query_status = (self.query_mode)(
            // The pointer to the GOP instance
            self,
            // The mode number associated with the mode we want information about
            mode_number,
            // The size of the `GraphicsModeInfo` structure
            &size_of_info as *const _,
            // The pointer to the location to be mutated to hold the pointer to the
            // `GraphicsModeInfo` instance associated with the current `mode_number`
            // on a successful function execution
            &mut mode as *mut _,
        );
        if query_status == STATUS_SUCCESS {
            // Return a reference to the mode info
            Ok(unsafe { &*mode })
        } else {
            // Return the failure status
            Err(query_status)
        }
    }

    // Set a mode to the mode with the mode number desired_mode
    pub fn set_mode(&self, desired_mode: u32) -> Result<(), Status> {
        let set_mode_status = (self.set_mode)(self, desired_mode);
        if set_mode_status == STATUS_SUCCESS {
            Ok(())
        } else {
            Err(set_mode_status)
        }
    }
}

// The blueprint to intepret the bits in **info upon a successful return from calling the
// GraphicsOutput's `query_mode` function
#[repr(C)]
pub struct GraphicsModeInfo {
    // The UEFI version number of this data structure
    pub version: u32,
    // The number of pixels that can be contained in one
    // horizontal row of the video screen in the mode whose info was requested
    pub horizontal_resolution: u32,
    // The number of pixels that can be contained in one vertical
    // column of the video screen in this mode whose info was requested
    pub vertical_resolution: u32,
    // Indicates how the bits of representing a single pixel should
    // be interpreted
    pub pixel_format: PixelFormat,
    // Some value whose meaning depends on the value of `pixel_format`
    pub pixel_info: PixelBitmask,
    // The number of pixels in one line of video memory.
    // Similar to `horizontal_resolution`, but different in a few way I think
    // are irrelevant
    pub pixels_per_scan_line: u32,
}

// Defines how to interpret the bits that represent a single pixel
#[derive(PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum PixelFormat {
    RedGreenBlueReserved = 0,
    BlueGreenRedReserved = 1,
    BitMask = 2,
    BltOnly = 3,
}

// A description of the color channels of a pixel in the GOP's framebuffer
#[repr(C)]
pub struct Pixel {
    // The bits representing the blue color intensity in this pixel
    pub blue: u8,
    // The bits representing the green color intensity in this pixel
    pub green: u8,
    // The bits representing the red color intensity in this pixel
    pub red: u8,
    // Unused bits
    pub reserved: u8,
}

// A structure telling how to re-interpret the bits in a pixel instance
// when the `GraphicsModeInfo` instance is set to `PixelFormat::BIT_MASK`
#[repr(C)]
pub struct PixelBitmask {
    // The bits set to 1 in this field tells which bits in a pixel should be
    // interpreted as the red color intensity when the `GraphicsModeInfo` instance
    // is set to `PixelFormat::BIT_MASK`
    pub red_mask: u32,
    // The bits set to 1 in this field tells which bits in a pixel should be
    // interpreted as the green color intensity when the `GraphicsModeInfo` instance
    // is set to `PixelFormat::BIT_MASK`
    pub green_mask: u32,
    // The bits set to 1 in this field tells which bits in a pixel should be
    // interpreted as the blue color intensity when the `GraphicsModeInfo` instance
    // is set to `PixelFormat::BIT_MASK`
    pub blue_mask: u32,
    // The bits set to 1 in this field tells which bits in a pixel should be
    // interpreted as the reserved field when the `GraphicsModeInfo` instance
    // is set to `PixelFormat::BIT_MASK`
    pub reserved: u32,
}

// The physical address on x86_64 is 8 bytes (64 bits)
pub type PhysAddr = u64;

// Gives info about the currently set and other available graphics modes
#[repr(C)]
pub struct GraphicsMode {
    // The number of modes supported by `GraphicsOutput::set_mode`
    // and `GraphicsOutput::query_mode`
    pub max_mode: u32,
    // The number associated with the current mode of the graphics
    // device. Valid values are always in the range 0..=`max_mode`-1
    pub mode: u32,
    // Pointer to a read only GraphicsModeInfo
    pub info: *const GraphicsModeInfo,
    // Size of the `GraphicsModeInfo` structure
    pub size_of_info: usize,
    // The starting address of the graphics framebuffer
    pub framebuffer_base: PhysAddr,
    // The size of the framebuffer in bytes
    pub framebuffer_size: usize,
}
