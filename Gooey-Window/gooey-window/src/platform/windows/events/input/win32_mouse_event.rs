use windows::Win32::Foundation::HWND;

use crate::platform::agnostic::input::{Modifiers, MouseButton};

#[derive(Debug)]
pub enum Win32MouseEvent {
    MouseMove { hwnd: HWND, x: i32, y: i32, modifiers: Modifiers },
    MouseEnter { hwnd: HWND },
    MouseLeave { hwnd: HWND },
    MouseButtonPressed { hwnd: HWND, button: MouseButton, modifiers: Modifiers },
    MouseButtonReleased { hwnd: HWND, button: MouseButton, modifiers: Modifiers },
    MouseDoubleClick { hwnd: HWND, button: MouseButton, modifiers: Modifiers },
}