use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::MSG};

use crate::platform::windows::events::input::{msg_classifier::{is_ime_message, is_keyboard_message, is_mouse_message, is_pointer_message, is_scroll_message, is_text_message, is_touch_message}, Win32IMEEvent, Win32KeyboardEvent, Win32MouseEvent, Win32PointerEvent, Win32ScrollEvent, Win32TextEvent, Win32TouchEvent};

#[derive(Debug)]
pub enum Win32InputEvent {
    KeyboardEvent { hwnd: HWND, event: Win32KeyboardEvent },
    MouseEvent { hwnd: HWND, event: Win32MouseEvent },
    ScrollEvent { hwnd: HWND, event: Win32ScrollEvent },
    PointerEvent { hwnd: HWND, event: Win32PointerEvent },
    TouchEvent { hwnd: HWND, event: Win32TouchEvent },
    TextInputEvent { hwnd: HWND, event: Win32TextEvent },
    IMEEvent { hwnd: HWND, event: Win32IMEEvent },
}

impl TryFrom<MSG> for Win32InputEvent {
    type Error = ();

    fn try_from(msg: MSG) -> Result<Self, Self::Error> {
        let hwnd = msg.hwnd;
        let kind = msg.message;

        if is_keyboard_message(kind) {
            Win32KeyboardEvent::try_from(msg).map(|event| Self::KeyboardEvent { hwnd, event })
        } else if is_mouse_message(kind) {
            Win32MouseEvent::try_from(msg).map(|event| Self::MouseEvent { hwnd, event })
        } else if is_scroll_message(kind) {
            Win32ScrollEvent::try_from(msg).map(|event| Self::ScrollEvent { hwnd, event })
        } else if is_pointer_message(kind) {
            Win32PointerEvent::try_from(msg).map(|event| Self::PointerEvent { hwnd, event })
        } else if is_touch_message(kind) {
            Win32TouchEvent::try_from(msg).map(|event| Self::TouchEvent { hwnd, event })
        } else if is_text_message(kind) {
            Win32TextEvent::try_from(msg).map(|event| Self::TextInputEvent { hwnd, event })
        } else if is_ime_message(kind) {
            Win32IMEEvent::try_from(msg).map(|event| Self::IMEEvent { hwnd, event })
        } else {
            Err(())
        }
    }
}