#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(panic_info_message)]

mod font;
use font::FONT;

mod uefi;
use uefi::{SystemTable, Screen, PixelFormat, Pixel, pre_graphics_print_str, print_str, printint, 
NO_OF_PIXELS_IN_A_ROW, BootServices};

mod bitmap;
use bitmap::{FileHeader, DIBHeader, ColorTable, Color, Bitmap, draw_bitmap, erase_bitmap};

// This needs to be imported so that the functions it defines may
// be used
use core::fmt::Write;

mod gdt;
use gdt::{Descriptor, GDT, SegmentRegister};

mod interrupts;

mod game;

static mut SCREEN: Option<&mut Screen> = None;

static mut GDT: Option<GDT> = None;

#[no_mangle]
extern "efiapi" fn efi_main(
    handle: *const core::ffi::c_void,
    sys_table: *mut SystemTable,
) -> usize {
    // Using a reference instead of a raw pointer
    let sys_table = unsafe { &*sys_table };
    // Getting a reference to the Boot Services from the System Table
    let boot_services = sys_table.boot_services();
    
    let init_graphics_result = init_graphics(boot_services);
    // Halt with error if graphics initialization failed
    if let Err(msg) = init_graphics_result {
        let simple_text_output = sys_table.simple_text_output();
        write!(simple_text_output, "{}", msg);
        loop {}
    }
    let screen = init_graphics_result.unwrap();
    
    init_screen(screen);
    
    let screen = get_screen().unwrap();

    // Exiting Boot Services to gain full control over the system
    boot_services.exit_boot_services(handle).unwrap();

    setup_gdt();

    game::blasterball(screen);
    
    // Returning 0 because the function expects it
    0
}

// Creates a new GDT and sets it up
fn setup_gdt() {
    let gdt: &mut GDT;
    unsafe {
        GDT = Some(GDT::new());
        gdt = GDT.as_mut().unwrap();
    }
    let cs = gdt.add_descriptor(Descriptor::code_segment()).unwrap();
    let ds = gdt.add_descriptor(Descriptor::data_segment()).unwrap();
    interrupts::disable_interrupts();
    let gdt_pointer = gdt.as_pointer();
    gdt.load(&gdt_pointer);
    interrupts::enable_interrupts();
    gdt::CS.set(cs);
    gdt::DS.set(ds);
    gdt::SS.set(ds);
}

fn init_graphics(boot_services: &'static BootServices) -> Result<&'static mut Screen, &'static str> {
    // Locate the Graphics Output Protocol
    let gop = boot_services.locate_gop();
    // If location failed, return error
    if gop.is_err() {
        return Err("Failed to locate GOP\n");
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
            return Err("Failed to query mode\n");
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
        // Return an error if the desired mode wasn't found
        if mode_number == max_mode - 1 {
            return Err("Failed to find desired mode\n");
        }
    }

    // Setting the mode to our desired mode
    let set_mode_result = gop.set_mode(desired_mode);

    // Checking if it is a success
    if set_mode_result.is_err() {
        return Err("Failed to set the desired mode\n");
    }

    // A reference to the screen
    Ok(mode.screen())

}

// Initializes the screen static
// This function is unsafe because the caller has to
// manually verify that the argument passed is a valid
// pointer to Screen
fn init_screen(screen: &'static mut Screen) {
    unsafe { SCREEN = Some(screen); }
}

pub fn get_screen() -> Option<&'static mut &'static mut Screen> {
    unsafe { SCREEN.as_mut() }
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let screen = get_screen().unwrap();
    write!(screen, "{}", panic_info);
    loop {}
}
