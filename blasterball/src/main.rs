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
use sync::{once::Once, mutex::Mutex};


static SCREEN: Mutex<Option<&mut Screen>> = Mutex::new(None);

static GDT: Once<GDT> = Once::new();
static TSS: TSS = TSS::new();
static IDT: Once<IDT> = Once::new();
static PICS: Mutex<PICs> = Mutex::new(PICs::new());
static KEYBOARD: Mutex<Keyboard> = Mutex::new(Keyboard::new());

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

    init_screen(init_graphics_result.unwrap());

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

    interrupts::disable_interrupts();
    setup_gdt();
    setup_idt();
    setup_pics();
    interrupts::enable_interrupts();

    event_hook::hook_event(EventKind::Keyboard, boxed_fn::BoxedFn::new(|event_info| {
        if let EventInfo::Keyboard(key_event) = event_info {
            if key_event.direction == keyboard::KeyDirection::Down {
                write!(SCREEN.lock().as_mut().unwrap(), "{:?}", key_event.keycode);
            }
        }
    }, allocator::get_allocator()));

    game::blasterball(&SCREEN);
    
    // Returning 0 because the function expects it
    0
}

// Creates a new GDT and sets it up
fn setup_gdt() {
    let (mut cs, mut ds, mut tss_sel) = (SegmentSelector(0), SegmentSelector(0), SegmentSelector(0));
    GDT.call_once(|| {
        let mut gdt = GDT::new();
        cs = gdt.add_descriptor(Descriptor::code_segment()).unwrap();
        ds = gdt.add_descriptor(Descriptor::data_segment()).unwrap();
        tss_sel = gdt.add_descriptor(Descriptor::tss_segment(&TSS)).unwrap();
        gdt
    });
    let gdt_pointer = GDT.as_pointer();
    GDT.load(&gdt_pointer);
    gdt::CS.set(cs);
    gdt::DS.set(ds);
    gdt::SS.set(ds);
    load_tss(tss_sel);
}

// Creates a new IDT and sets it up
fn setup_idt() {
    IDT.call_once(|| {
        let mut idt = IDT::new();
        let sel = gdt::CS.read();
        idt.breakpoint = Entry::exception(ServiceRoutine(breakpoint_handler), sel);
        idt.page_fault = Entry::exception(ServiceRoutineWithErrCode(page_fault_handler), sel);
        idt.double_fault = Entry::exception(ServiceRoutineWithNoReturn(double_fault_handler), sel);
        idt.interrupts[0] = Entry::interrupt(ServiceRoutine(timer_handler), sel);
        idt.interrupts[1] = Entry::interrupt(ServiceRoutine(keyboard_handler), sel);
        idt
    });
    let pointer = IDT.as_pointer();
    interrupts::load_idt(&pointer);
}

// Initializes the PICs
fn setup_pics() {
    PICS.lock().init();
}

extern "x86-interrupt" fn page_fault_handler(frame: interrupts::InterruptStackFrame, err_code: u64) {
    panic!("Page faulted with error code {}", err_code);
}

extern "x86-interrupt" fn breakpoint_handler(frame: interrupts::InterruptStackFrame) {
    write!(SCREEN.lock().as_mut().unwrap(), "In the breakpoint handler");
}

extern "x86-interrupt" fn double_fault_handler(frame: interrupts::InterruptStackFrame, err_code: u64) -> ! {
    panic!("Double fault with error code {}", err_code);
}

extern "x86-interrupt" fn timer_handler(frame: interrupts::InterruptStackFrame) {
    // Notifying the event hooker that the timer event has occured
    event_hook::send_event(EventInfo::Timer);
    // Signalling that the timer interrupt has been handled
    PICS.lock().end_of_interrupt(0);
}

extern "x86-interrupt" fn keyboard_handler(frame: interrupts::InterruptStackFrame) {
    let port = port::Port::new(0x60);
    let scancode = port.read();
    if let Ok(Some(event)) = KEYBOARD.lock().interpret_byte(scancode) {
        // Notifying the event hooker that the keyboard event has occured
        event_hook::send_event(EventInfo::Keyboard(event));
    }
    // Signalling that the keyboard interrupt has been handled
    PICS.lock().end_of_interrupt(1);
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
fn init_screen(screen: &'static mut Screen) {
    *SCREEN.lock() = Some(screen);
}

#[cfg_attr(not(test), panic_handler)]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    write!(SCREEN.lock().as_mut().unwrap(), "{}", panic_info);
    loop {}
}
