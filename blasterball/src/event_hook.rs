use crate::keyboard::KeyEvent;
use crate::vec::Vec;
use crate::boxed_fn::BoxedFn;
use crate::allocator::Allocator;


// Mediator between the game code and the interrupt service routines
pub struct EventHooker {
    // Functions to be called when the timer event occurs
    timer_handlers: Vec<Handler>,
    // Functions to be called when the keyboard event occurs
    keyboard_handlers: Vec<Handler>,
    // The identifier to be used for the next function added
    next_id: usize
}

// The info about an event that will be passed into a handler
// when it is called
#[derive(Clone, Copy)]
pub enum EventInfo {
    Timer,
    Keyboard(KeyEvent)
}

#[derive(Clone)]
struct Handler {
    // The identifier for this handler
    id: HandlerId,
    // A function called when an event occurs
    func: BoxedFn
}

pub enum EventKind {
    Timer,
    Keyboard
}

type HandlerId = usize;

impl EventHooker {
    // Creates a new EventHooker instance
    pub fn new(allocator: *mut dyn Allocator) -> Self {
        EventHooker {
            timer_handlers: Vec::with_capacity(10, allocator),
            keyboard_handlers: Vec::with_capacity(10, allocator),
            next_id: 0
        }
    }

    // Used by client code to tell the EventHooker to call function `func`
    // whenever an event of the kind `event_kind` takes place.
    // The returned ID is used to identify the handler when removing it
    pub fn hook_event(&mut self, event_kind: EventKind, func: BoxedFn) -> HandlerId {
        let handler_vec = match event_kind {
            EventKind::Timer => &mut self.timer_handlers,
            EventKind::Keyboard => &mut self.keyboard_handlers
        };
        let new_handler = Handler { func, id: self.next_id };
        handler_vec.push(new_handler);
        self.next_id += 1;
        handler_vec[handler_vec.len() - 1].id
    }

    // Used by Interrupt Service Routines to tell the EventHooker
    // that an event has occured. The `event_info` provides information
    // about the event that is passed to the handler function
    pub fn send_event(&self, event_info: EventInfo) {
        let handler_vec = match event_info {
            EventInfo::Timer => &self.timer_handlers,
            EventInfo::Keyboard(_) => &self.keyboard_handlers
        };
        for handler in handler_vec.iter() {
            (handler.func)(event_info);
        }
    }

    // Used by client code to tell the EventHooker to stop calling a handler with
    // with ID `id` when an event of kind `event_kind` occurs.
    // A return value of `Ok(())` means that the handler was found and removed.
    // A return value of `Err(())` means that the handler was not found.
    pub fn unhook_event(&mut self, event_kind: EventKind, id: HandlerId) -> Result<(), ()> {
        let handler_vec = match event_kind {
            EventKind::Timer => &mut self.timer_handlers,
            EventKind::Keyboard => &mut self.keyboard_handlers
        };
        for i in 0..handler_vec.len() {
            if handler_vec[i].id == id {
                handler_vec.remove(i);
                return Ok(())
            }
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::Allocator;
    use crate::keyboard::{KeyCode, KeyDirection};

    #[test]
    fn test_event_hooker1() {
        let allocator = successful_allocator();
        let mut event_hooker = EventHooker::new(allocator);
        // This variable is increased everytime a timer handler is called
        let mut timer_no = 0;
        // This variable is increased everytime a keyboard handler is called
        let mut keyboard_no = 0;
        // Registering the timer and keyboard handlers
        let timer_handler_hook_id = event_hooker.hook_event(
            EventKind::Timer,
            BoxedFn::new(|_| timer_no += 1, allocator)
        );
        let keyboard_handler_hook_id = event_hooker.hook_event(
            EventKind::Keyboard,
            BoxedFn::new(|_| keyboard_no += 1, allocator)
        );
        // Send the timer event 3 times
        for _ in 0..3 {
            event_hooker.send_event(EventInfo::Timer);
        }
        // Since the timer event was sent 3 times and no keyboard
        // event has been sent, then the timer_no should have increased
        // to 3 and the keyboard_no should have remained 0
        assert_eq!(timer_no, 3);
        assert_eq!(keyboard_no, 0);
        // Send the keyboard event 3 times
        for _ in 0..3 {
            event_hooker.send_event(EventInfo::Keyboard(KeyEvent {
                keycode: KeyCode::A,
                direction: KeyDirection::Down
            }));
        }
        // The timer_no should still be 3 and the keyboard_no should increase to 3
        assert_eq!(timer_no, 3);
        assert_eq!(keyboard_no, 3);
    }

    #[test]
    fn test_event_hooker2() {
        let allocator = successful_allocator();
        let mut event_hooker = EventHooker::new(allocator);
        // The number of times the timer handler was called
        let mut no_of_calls = 0;
        let hook_id = event_hooker.hook_event(
            EventKind::Timer,
            BoxedFn::new(|_| no_of_calls += 1, allocator)
        );
        // Send the timer event 5 times
        for _ in 0..5 {
            event_hooker.send_event(EventInfo::Timer);
        }
        // Verify that no_of_calls increased to 5
        // (no_of_calls is incremented with every invocation
        // of the timer handler)
        assert_eq!(no_of_calls, 5);
        // Unregister the timer handler
        event_hooker.unhook_event(EventKind::Timer, hook_id).unwrap();
        // Send the timer event 5 times and verify that
        // no_of_calls remained the same
        for _ in 0..5 {
            event_hooker.send_event(EventInfo::Timer);
        }
        assert_eq!(no_of_calls, 5);
    }

    // Convenience function for getting the always successful allocator
    fn successful_allocator() -> *mut SuccessfulAllocator {
        &mut SuccessfulAllocator as *mut _
    }

        // Convenience function for getting the always fail allocator
    fn failing_allocator() -> *mut FailingAllocator {
        &mut FailingAllocator as *mut _
    }

    // Dummy allocator that we can depend on to always succeed
    struct SuccessfulAllocator;

    
    use std::alloc::Global as PlatformAllocator;
    use std::alloc::Layout;
    use std::ptr::NonNull;
    use std::alloc::Allocator as StdAllocator;
    
    // Use your computer's allocator to allocate and deallocate memory
    // Much more reliable than using our own custom allocator,
    // so we can depend on it succeeding (under normal circumstances)
    unsafe impl Allocator for SuccessfulAllocator {
        unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Option<*mut u8> {
            let mem_layout = Layout::from_size_align(size, alignment).unwrap();
            let mem = PlatformAllocator.allocate(mem_layout).unwrap();
            let ptr = mem.as_ptr() as *mut u8;
            Some(ptr)
        }
        unsafe fn dealloc(&mut self, ptr: *mut u8, size_to_dealloc: usize) {
            // Using an alignment of 1 here because I think the alignment no
            // longer matters here. We're deallocating memory because we're
            // done using it
            let mem_layout = Layout::from_size_align(size_to_dealloc, 1).unwrap();
            PlatformAllocator.deallocate(NonNull::new(ptr).unwrap(), mem_layout);
        }
    }

    // Dummy allocator we can depend on to always fail
    struct FailingAllocator;

    unsafe impl Allocator for FailingAllocator {
        unsafe fn alloc(&mut self, size: usize, alignment: usize) -> Option<*mut u8> {
            None
        }
        unsafe fn dealloc(&mut self, ptr: *mut u8, size_to_dealloc: usize) {}
    }
}