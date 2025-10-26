use windows::Win32::UI::WindowsAndMessaging::{WM_CHAR, WM_IME_COMPOSITION, WM_IME_ENDCOMPOSITION, WM_IME_NOTIFY, WM_IME_REQUEST, WM_IME_SETCONTEXT, WM_IME_STARTCOMPOSITION, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MBUTTONDOWN, WM_MBUTTONUP, WM_MOUSEHWHEEL, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_POINTERUPDATE, WM_POINTERWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP, WM_SYSKEYDOWN, WM_SYSKEYUP, WM_TOUCH, WM_UNICHAR, WM_XBUTTONDOWN, WM_XBUTTONUP};

pub fn is_keyboard_message(msg: u32) -> bool {
    matches!(msg, WM_KEYDOWN | WM_KEYUP | WM_SYSKEYDOWN | WM_SYSKEYUP)
}

pub fn is_mouse_message(msg: u32) -> bool {
    matches!(
        msg,
        WM_MOUSEMOVE |
        WM_LBUTTONDOWN | WM_LBUTTONUP |
        WM_RBUTTONDOWN | WM_RBUTTONUP |
        WM_MBUTTONDOWN | WM_MBUTTONUP |
        WM_XBUTTONDOWN | WM_XBUTTONUP
    )
}

pub fn is_scroll_message(msg: u32) -> bool {
    matches!(msg, WM_MOUSEWHEEL | WM_MOUSEHWHEEL)
}

pub fn is_pointer_message(msg: u32) -> bool {
    (WM_POINTERUPDATE..=WM_POINTERWHEEL).contains(&msg)
}

pub fn is_touch_message(msg: u32) -> bool {
    msg == WM_TOUCH
}

pub fn is_text_message(msg: u32) -> bool {
    matches!(msg, WM_CHAR | WM_UNICHAR)
}

pub fn is_ime_message(msg: u32) -> bool {
    matches!(
        msg,
        WM_IME_STARTCOMPOSITION | WM_IME_ENDCOMPOSITION |
        WM_IME_COMPOSITION | WM_IME_NOTIFY |
        WM_IME_SETCONTEXT | WM_IME_REQUEST
    )
}