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