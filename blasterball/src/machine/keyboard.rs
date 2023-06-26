const EXTENDED_CODE: u8 = 0xe0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyDirection {
    // The key is being released
    Up,
    // The key is being pressed down
    Down
}

// A code associated with a particular key on the keyboard in scancode set 1
#[derive(Debug, Clone, Copy)]
pub enum KeyCode {
    Escape,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Dash,
    Equals,
    Backspace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    OpenBracket,
    CloseBracket,
    Enter,
    LeftCtrl,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    SemiColon,
    SingleQuote,
    Backtick,
    LeftShift,
    BackSlash,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    ForwardSlash,
    RightShift,
    KeypadStar,
    LeftAlt,
    Space,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    NumLock,
    ScrollLock,
    KeypadSeven,
    KeypadEight,
    KeypadNine,
    KeypadDash,
    KeypadFour,
    KeypadFive,
    KeypadSix,
    KeypadOne,
    KeypadTwo,
    KeypadThree,
    KeypadPlus,
    KeypadZero,
    KeypadDot,
    PrevTrack,
    NextTrack,
    KeypadEnter,
    RightCtrl,
    Mute,
    Calculator,
    Play,
    Stop,
    VolumeDown,
    VolumeUp,
    WWWHome,
    KeypadForwardSlash,
    AltGr,
    Home,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    PageUp,
    End,
    PageDown,
    Insert,
    Delete,
    LeftGUI,
    RightGUI,
    Apps,
    AcpiPower,
    AcpiSleep,
    AcpiWake,
    WWWSearch,
    WWWFavorites,
    WWWRefresh,
    WWWStop,
    WWWForward,
    WWWBack,
    MyComputer,
    Email,
    MediaSelect
}

#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub keycode: KeyCode,
    pub direction: KeyDirection
}

pub struct Keyboard {
    // Tells if the next scancode to be interpreted
    // follows an extended scancode
    code_is_extended: bool
}

impl Keyboard {
    // Creates a new instance of the Keyboard
    pub const fn new() -> Self {
        // Initially, the extended keycode has not been read
        Self { code_is_extended: false }
    }

    // Accepts a byte and changes the keyboard state in the case of beginning 
    // or end of an extended code.
    // Else, just returns the event associated with the scancode byte.
    pub fn interpret_byte(&mut self, scancode: u8) -> Result<Option<KeyEvent>, ()> {
        match self.code_is_extended {
            // scancode does not follow the extended scancode
            false => {
                match scancode {
                    EXTENDED_CODE => {
                        // The next scancode follows the extended code
                        self.code_is_extended = true;
                        // The extended code can't be translated to an event
                        Ok(None)
                    }
                    // The scancode range for regular key presses
                    0x01..=0x58 => {
                        let keycode = self.map_scancode(scancode)?;
                        Ok(Some(KeyEvent {
                            keycode,
                            direction: KeyDirection::Down
                        }))
                    }
                    // The scancode range for regular key releases
                    0x81..=0xd8 => {
                        // key up scancodes are just 0x80 added to their counter
                        // key down codes
                        let keycode = self.map_scancode(scancode - 0x80)?;
                        Ok(Some(KeyEvent {
                            keycode,
                            direction: KeyDirection::Up
                        }))
                    }
                    // Invalid scancode
                    _ => Err(())
                }
            }
            // scancode follows the extended scancode
            true => {
                self.code_is_extended = false;
                match scancode {
                    // Range of scancodes for extended key presses
                    0x10..=0x90 => {
                        let keycode = self.map_extended_scancode(scancode)?;
                        Ok(Some(KeyEvent {
                            keycode,
                            direction: KeyDirection::Down
                        }))
                    }
                    // Range for extended key releases
                    0x99..=0xed => {
                        // key up scancodes are just 0x80 added to their counter
                        // key down codes
                        let keycode = self.map_extended_scancode(scancode - 0x80)?;
                        Ok(Some(KeyEvent {
                            keycode,
                            direction: KeyDirection::Up
                        }))
                    }
                    // Invalid scancode
                    _ => Err(())
                }
            }
        }
    }

    // Takes a scancode that doesn't follow 0x0e and returns
    // its KeyCode, in the case of a success.
    // If the scancode is invalid, a Err(()) is returned
    fn map_scancode(&self, scancode: u8) -> Result<KeyCode, ()> {
        match scancode {
            0x01 => Ok(KeyCode::Escape),
            0x02 => Ok(KeyCode::One),
            0x03 => Ok(KeyCode::Two),
            0x04 => Ok(KeyCode::Three),
            0x05 => Ok(KeyCode::Four),
            0x06 => Ok(KeyCode::Five),
            0x07 => Ok(KeyCode::Six),
            0x08 => Ok(KeyCode::Seven),
            0x09 => Ok(KeyCode::Eight),
            0x0a => Ok(KeyCode::Nine),
            0x0b => Ok(KeyCode::Zero),
            0x0c => Ok(KeyCode::Dash),
            0x0d => Ok(KeyCode::Equals),
            0x0e => Ok(KeyCode::Backspace),
            0x0f => Ok(KeyCode::Tab),
            0x10 => Ok(KeyCode::Q),
            0x11 => Ok(KeyCode::W),
            0x12 => Ok(KeyCode::E),
            0x13 => Ok(KeyCode::R),
            0x14 => Ok(KeyCode::T),
            0x15 => Ok(KeyCode::Y),
            0x16 => Ok(KeyCode::U),
            0x17 => Ok(KeyCode::I),
            0x18 => Ok(KeyCode::O),
            0x19 => Ok(KeyCode::P),
            0x1a => Ok(KeyCode::OpenBracket),
            0x1b => Ok(KeyCode::CloseBracket),
            0x1c => Ok(KeyCode::Enter),
            0x1d => Ok(KeyCode::LeftCtrl),
            0x1e => Ok(KeyCode::A),
            0x1f => Ok(KeyCode::S),
            0x20 => Ok(KeyCode::D),
            0x21 => Ok(KeyCode::F),
            0x22 => Ok(KeyCode::G),
            0x23 => Ok(KeyCode::H),
            0x24 => Ok(KeyCode::J),
            0x25 => Ok(KeyCode::K),
            0x26 => Ok(KeyCode::L),
            0x27 => Ok(KeyCode::SemiColon),
            0x28 => Ok(KeyCode::SingleQuote),
            0x29 => Ok(KeyCode::Backtick),
            0x2a => Ok(KeyCode::LeftShift),
            0x2b => Ok(KeyCode::BackSlash),
            0x2c => Ok(KeyCode::Z),
            0x2d => Ok(KeyCode::X),
            0x2e => Ok(KeyCode::C),
            0x2f => Ok(KeyCode::V),
            0x30 => Ok(KeyCode::B),
            0x31 => Ok(KeyCode::N),
            0x32 => Ok(KeyCode::M),
            0x33 => Ok(KeyCode::Comma),
            0x34 => Ok(KeyCode::Dot),
            0x35 => Ok(KeyCode::ForwardSlash),
            0x36 => Ok(KeyCode::RightShift),
            0x37 => Ok(KeyCode::KeypadStar),
            0x38 => Ok(KeyCode::LeftAlt),
            0x39 => Ok(KeyCode::Space),
            0x3a => Ok(KeyCode::CapsLock),
            0x3b => Ok(KeyCode::F1),
            0x3c => Ok(KeyCode::F2),
            0x3d => Ok(KeyCode::F3),
            0x3e => Ok(KeyCode::F4),
            0x3f => Ok(KeyCode::F5),
            0x40 => Ok(KeyCode::F6),
            0x41 => Ok(KeyCode::F7),
            0x42 => Ok(KeyCode::F8),
            0x43 => Ok(KeyCode::F9),
            0x44 => Ok(KeyCode::F10),
            0x57 => Ok(KeyCode::F11),
            0x58 => Ok(KeyCode::F12),
            0x45 => Ok(KeyCode::NumLock),
            0x46 => Ok(KeyCode::ScrollLock),
            0x47 => Ok(KeyCode::KeypadSeven),
            0x48 => Ok(KeyCode::KeypadEight),
            0x49 => Ok(KeyCode::KeypadNine),
            0x4a => Ok(KeyCode::KeypadDash),
            0x4b => Ok(KeyCode::KeypadFour),
            0x4c => Ok(KeyCode::KeypadFive),
            0x4d => Ok(KeyCode::KeypadSix),
            0x4e => Ok(KeyCode::KeypadPlus),
            0x4f => Ok(KeyCode::KeypadOne),
            0x50 => Ok(KeyCode::KeypadTwo),
            0x51 => Ok(KeyCode::KeypadThree),
            0x52 => Ok(KeyCode::KeypadZero),
            0x53 => Ok(KeyCode::KeypadDot),
            // Invalid scancode
            _ => Err(())
        }
    }

    // Takes a scancode that doesn't follow 0x0e and returns
    // its KeyCode, in the case of a success.
    // If the scancode is invalid, a Err(()) is returned
    fn map_extended_scancode(&self, scancode: u8) -> Result<KeyCode, ()> {
        match scancode {
            0x10 => Ok(KeyCode::PrevTrack),
            0x19 => Ok(KeyCode::NextTrack),
            0x1c => Ok(KeyCode::KeypadEnter),
            0x1d => Ok(KeyCode::RightCtrl),
            0x20 => Ok(KeyCode::Mute),
            0x21 => Ok(KeyCode::Calculator),
            0x22 => Ok(KeyCode::Play),
            0x24 => Ok(KeyCode::Stop),
            0x2e => Ok(KeyCode::VolumeDown),
            0x30 => Ok(KeyCode::VolumeUp),
            0x32 => Ok(KeyCode::WWWHome),
            0x35 => Ok(KeyCode::KeypadForwardSlash),
            0x38 => Ok(KeyCode::AltGr),
            0x47 => Ok(KeyCode::Home),
            0x48 => Ok(KeyCode::ArrowUp),
            0x49 => Ok(KeyCode::PageUp),
            0x4b => Ok(KeyCode::ArrowLeft),
            0x4d => Ok(KeyCode::ArrowRight),
            0x4f => Ok(KeyCode::End),
            0x50 => Ok(KeyCode::ArrowDown),
            0x51 => Ok(KeyCode::PageDown),
            0x52 => Ok(KeyCode::Insert),
            0x53 => Ok(KeyCode::Delete),
            0x5b => Ok(KeyCode::LeftGUI),
            0x5c => Ok(KeyCode::RightGUI),
            0x5d => Ok(KeyCode::Apps),
            0x5e => Ok(KeyCode::AcpiPower),
            0x5f => Ok(KeyCode::AcpiSleep),
            0x63 => Ok(KeyCode::AcpiWake),
            0x65 => Ok(KeyCode::WWWSearch),
            0x66 => Ok(KeyCode::WWWFavorites),
            0x67 => Ok(KeyCode::WWWRefresh),
            0x68 => Ok(KeyCode::WWWStop),
            0x69 => Ok(KeyCode::WWWForward),
            0x6a => Ok(KeyCode::WWWBack),
            0x6b => Ok(KeyCode::MyComputer),
            0x6c => Ok(KeyCode::Email),
            0x6d => Ok(KeyCode::MediaSelect),
            _ => Err(())
        }
    }
}
