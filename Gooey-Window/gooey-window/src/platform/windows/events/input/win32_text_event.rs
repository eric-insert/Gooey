use windows::Win32::Foundation::HWND;

use crate::platform::agnostic::input::Modifiers;

#[derive(Debug)]
pub enum Win32TextEvent {
    CharInput { hwnd: HWND, char_code: char, modifiers: Modifiers },
    UnicodeInput { hwnd: HWND, code_point: u32, modifiers: Modifiers },
}