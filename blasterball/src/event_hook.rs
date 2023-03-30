// Mediator between the game code and the interrupt service routines
pub struct EventHooker {
    // The functions that will be called when an event occurs
    handlers: Vec<Box<dyn FnMut(EventInfo)>>
}

// The info about an event that will be passed into a handler
// when it is called
pub enum EventInfo {
    Timer,
    Keyboard(KeyEvent)
}