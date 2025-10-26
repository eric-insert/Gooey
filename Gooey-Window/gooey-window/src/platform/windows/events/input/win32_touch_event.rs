use windows::Win32::Foundation::HWND;

use crate::platform::agnostic::input::Modifiers;

#[derive(Debug)]
pub enum Win32TouchEvent {
    TouchDown { hwnd: HWND, id: u32, x: i32, y: i32, modifiers: Modifiers },
    TouchMove { hwnd: HWND, id: u32, x: i32, y: i32, modifiers: Modifiers },
    TouchUp { hwnd: HWND, id: u32, x: i32, y: i32, modifiers: Modifiers },
    TouchCancel { hwnd: HWND, id: u32 },
    GestureBegin { hwnd: HWND },
    GestureEnd { hwnd: HWND },
    Zoom { hwnd: HWND, scale: f32, center_x: i32, center_y: i32 },
    Rotate { hwnd: HWND, angle_delta: f32, center_x: i32, center_y: i32 },
    Pan { hwnd: HWND, delta_x: i32, delta_y: i32 },
}