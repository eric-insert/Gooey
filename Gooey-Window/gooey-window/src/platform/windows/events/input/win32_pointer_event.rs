use windows::Win32::Foundation::HWND;

use crate::platform::agnostic::input::{Modifiers, MouseButton};

#[derive(Debug)]
pub enum Win32PointerEvent {
    PointerDown { hwnd: HWND, id: u32, x: i32, y: i32, button: Option<MouseButton>, modifiers: Modifiers },
    PointerUp { hwnd: HWND, id: u32, x: i32, y: i32, button: Option<MouseButton>, modifiers: Modifiers },
    PointerMove {hwnd: HWND, id: u32, x: i32, y: i32, modifiers: Modifiers },
    PointerEnter { hwnd: HWND, id: u32 },
    PointerLeave { hwnd: HWND, id: u32 },
    PointerWheel { hwnd: HWND, id: u32, delta: i16, modifiers: Modifiers },
    PointerHWheel {hwnd: HWND, id: u32, delta: i16, modifiers: Modifiers },
}