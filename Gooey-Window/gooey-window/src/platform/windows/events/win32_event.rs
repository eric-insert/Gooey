use windows::Win32::{Foundation::{HWND, LPARAM, WPARAM}, UI::{Input::HRAWINPUT, WindowsAndMessaging::{MSG, WM_CHAR, WM_CLOSE, WM_DESTROY, WM_KEYDOWN, WM_KEYUP, WM_LBUTTONDOWN, WM_LBUTTONUP, WM_MOUSEMOVE, WM_MOUSEWHEEL, WM_RBUTTONDOWN, WM_RBUTTONUP}}};

use crate::platform::{agnostic::input::{Modifiers, MouseButton}, windows::{events::{input::Win32InputEvent, win32_file_event::Win32FileEvent, win32_window_event::Win32WindowEvent}, utils::{get_modifiers, get_x_lparam, get_y_lparam}}};

#[derive(Debug)]
pub enum Win32Event {
    WindowEvent { hwnd: HWND, event: Win32WindowEvent },
    InputEvent { hwnd: HWND, event: Win32InputEvent },
    FileEvent { hwnd: HWND, event: Win32FileEvent },

    // --- Custom / fallback ---
    AppMessage { hwnd: HWND, message: u32, wparam: WPARAM, lparam: LPARAM },
}



// ERIC YOU ARE CURRENTLY CODING THE TRY FROM FOR EACH OF THE ENUMS. YOU LEFT OFF ON INPUT! I LOVE YOU BUDDY <3 


impl TryFrom<MSG> for Win32Event {
    type Error = ();

    fn try_from(msg: MSG) -> Result<Self, Self::Error> {
        let hwnd = msg.hwnd;

        if let Ok(event) = Win32WindowEvent::try_from(msg) {
            return Ok(Self::WindowEvent { hwnd, event });
        }

        if let Ok(event) = Win32InputEvent::try_from(msg) {
            return Ok(Self::InputEvent { hwnd, event });
        }

        if let Ok(event) = Win32FileEvent::try_from(msg) {
            return Ok(Self::FileEvent { hwnd, event });
        }

        // Fallback for unknown/custom messages
        Ok(Self::AppMessage {
            hwnd,
            message: msg.message,
            wparam: msg.wParam,
            lparam: msg.lParam,
        })
    }
}