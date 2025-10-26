#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum KeyCode {
    // Alphanumeric
    A = 0, 
    B, 
    C, 
    D, 
    E, 
    F, 
    G, 
    H, 
    I, 
    J,
    K, 
    L, 
    M, 
    N, 
    O, 
    P, 
    Q, 
    R, 
    S, 
    T,
    U, 
    V, 
    W, 
    X, 
    Y, 
    Z,
    Key0, 
    Key1, 
    Key2, 
    Key3, 
    Key4,
    Key5, 
    Key6, 
    Key7, 
    Key8, 
    Key9,

    // Function keys
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

    // Modifiers (used as raw keys, not modifier *state*)
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
    SuperLeft,
    SuperRight,

    // Navigation
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,

    // Editing
    Backspace,
    Delete,
    Insert,
    Enter,
    Escape,
    Tab,
    Space,

    // Symbols
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Quote,
    Backtick,
    Comma,
    Period,
    Slash,

    // Lock keys
    CapsLock,
    NumLock,
    ScrollLock,

    // Keypad
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadSubtract,
    NumpadMultiply,
    NumpadDivide,
    NumpadDecimal,
    NumpadEnter,
    __COUNT,
}

impl KeyCode {
    pub fn to_index(self) -> Option<usize> {
        match self {
            _ => Some(self as u16 as usize),
        }
    }

    pub const COUNT: usize = KeyCode::__COUNT as usize;
}