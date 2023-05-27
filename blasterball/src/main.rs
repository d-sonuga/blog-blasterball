#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(abi_efiapi, abi_x86_interrupt)]
#![feature(panic_info_message)]
#![feature(unboxed_closures, fn_traits)]
#![cfg_attr(test, feature(allocator_api))]

mod machine;
mod alloc;
mod display;
mod event_hook;
mod game;
mod sync;

use core::fmt::Write;
use machine::{
    port,
    uefi::{SystemTable, BootServices, Screen, MemType, PixelFormat},
    tss,
    keyboard,
    keyboard::Keyboard,
    pics::PICs,
    gdt,
    gdt::{GDT, Descriptor, SegmentSelector, SegmentRegister},
    tss::{TSS, load_tss},
    interrupts,
    interrupts::{IDT, ServiceRoutine, Entry, ServiceRoutineWithErrCode, ServiceRoutineWithNoReturn}
};
use alloc::{
    allocator,
    boxed_fn
};
use event_hook::{EventKind, EventInfo};


static mut SCREEN: Option<&mut Screen> = None;

static mut GDT: Option<GDT> = None;
static mut TSS: Option<TSS> = None;
static mut IDT: Option<IDT> = None;
static mut PICS: Option<PICs> = None;
static mut KEYBOARD: Option<Keyboard> = None;

// A heap size of 1 megabyte
const HEAP_SIZE: usize = 2usize.pow(20);

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

    // Allocating memory for the heap
    let heap_mem = boot_services.alloc_pool(MemType::EfiLoaderData, HEAP_SIZE);
    if heap_mem.is_err() {
        panic!("Heap allocation failed with error status {}", heap_mem.unwrap_err());
    }

    // Initializes the allocator with the heap memory
    allocator::init(allocator::MemChunk {
        start_addr: heap_mem.unwrap() as usize,
        size: HEAP_SIZE
    });

    event_hook::init();

    // Exiting Boot Services to gain full control over the system
    boot_services.exit_boot_services(handle).unwrap();

    let cs = setup_gdt();
    
    setup_idt(cs);
    setup_keyboard();
    setup_pics();

    event_hook::hook_event(EventKind::Keyboard, boxed_fn::BoxedFn::new(|event_info| {
        if let EventInfo::Keyboard(key_event) = event_info {
            if key_event.direction == keyboard::KeyDirection::Down {
                write!(screen, "{:?}", key_event.keycode);
            }
        }
    }, allocator::get_allocator()));

    game::blasterball(screen);
    
    // Returning 0 because the function expects it
    0
}

// Creates a new TSS and sets it up
fn setup_tss() {
    let tss: &mut TSS;
    unsafe {
        TSS = Some(TSS::new());
        tss = TSS.as_mut().unwrap();
    }
    let gdt = unsafe { GDT.as_mut().unwrap() };
    let tss_selector = gdt.add_descriptor(Descriptor::tss_segment(tss)).unwrap();
    load_tss(tss_selector);
}

// Creates a new GDT and sets it up
fn setup_gdt() -> SegmentSelector {
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
    cs
}

// Creates a new IDT and sets it up
fn setup_idt(sel: SegmentSelector) {
    let idt: &mut IDT;
    unsafe {
        IDT = Some(IDT::new());
        idt = IDT.as_mut().unwrap();
    }
    idt.breakpoint = Entry::exception(ServiceRoutine(breakpoint_handler), sel);
    idt.page_fault = Entry::exception(ServiceRoutineWithErrCode(page_fault_handler), sel);
    idt.double_fault = Entry::exception(ServiceRoutineWithNoReturn(double_fault_handler), sel);
    idt.interrupts[0] = Entry::interrupt(ServiceRoutine(timer_handler), sel);
    idt.interrupts[1] = Entry::interrupt(ServiceRoutine(keyboard_handler), sel);
    let pointer = idt.as_pointer();
    interrupts::disable_interrupts();
    interrupts::load_idt(&pointer);
    interrupts::enable_interrupts();
}

// Creates a new PICs instance and initializes it
fn setup_pics() {
    let pics: &mut PICs;
    unsafe {
        PICS = Some(PICs::new());
        pics = PICS.as_mut().unwrap();
    }
    pics.init();
}

fn setup_keyboard() {
    unsafe {
        KEYBOARD = Some(Keyboard::new());
    }
}

extern "x86-interrupt" fn page_fault_handler(frame: interrupts::InterruptStackFrame, err_code: u64) {
    panic!("Page faulted with error code {}", err_code);
}

extern "x86-interrupt" fn breakpoint_handler(frame: interrupts::InterruptStackFrame) {
    let screen = get_screen().unwrap();
    write!(screen, "In the breakpoint handler");
}

extern "x86-interrupt" fn double_fault_handler(frame: interrupts::InterruptStackFrame, err_code: u64) -> ! {
    panic!("Double fault with error code {}", err_code);
}

extern "x86-interrupt" fn timer_handler(frame: interrupts::InterruptStackFrame) {
    let screen = get_screen().unwrap();
    // Notifying the event hooker that the timer event has occured
    event_hook::send_event(EventInfo::Timer);
    // Signalling that the timer interrupt has been handled
    get_pics().unwrap().end_of_interrupt(0);
}

extern "x86-interrupt" fn keyboard_handler(frame: interrupts::InterruptStackFrame) {
    let port = port::Port::new(0x60);
    let scancode = port.read();
    if let Ok(Some(event)) = get_keyboard().unwrap().interpret_byte(scancode) {
        // Notifying the event hooker that the keyboard event has occured
        event_hook::send_event(EventInfo::Keyboard(event));
    }
    // Signalling that the keyboard interrupt has been handled
    get_pics().unwrap().end_of_interrupt(1);
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

pub fn get_pics() -> Option<&'static mut PICs> {
    unsafe { PICS.as_mut() }
}

fn get_keyboard() -> Option<&'static mut Keyboard> {
    unsafe { KEYBOARD.as_mut() }
}

#[cfg_attr(not(test), panic_handler)]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let screen = get_screen().unwrap();
    write!(screen, "{}", panic_info);
    loop {}
}
