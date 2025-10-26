use windows::Win32::Foundation::HWND;

use crate::platform::agnostic::input::Modifiers;

#[derive(Debug)]
pub enum Win32ScrollEvent {
    MouseWheel { hwnd: HWND, delta: i16, modifiers: Modifiers },
    MouseHWheel { hwnd: HWND, delta: i16, modifiers: Modifiers },
}