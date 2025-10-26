use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::{MSG, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP}};

use crate::platform::{agnostic::input::Modifiers, windows::utils::get_modifiers};

#[derive(Debug)]
pub enum Win32KeyboardEvent {
    KeyPressed { hwnd: HWND, vk_code: u32, modifiers: Modifiers },
    KeyReleased { hwnd: HWND, vk_code: u32, modifiers: Modifiers },
    SystemKeyPressed { hwnd: HWND, vk_code: u32, modifiers: Modifiers },
    SystemKeyReleased { hwnd: HWND, vk_code: u32, modifiers: Modifiers },
}

impl TryFrom<MSG> for Win32KeyboardEvent {
    type Error = ();

    fn try_from(msg: MSG) -> Result<Self, Self::Error> {
        let hwnd = msg.hwnd;
        let wparam = msg.wParam;
        let vk_code = wparam.0 as u32;
        let modifiers = get_modifiers();

        match msg.message {
            WM_KEYDOWN => Ok(Self::KeyPressed { hwnd, vk_code, modifiers }),
            WM_KEYUP => Ok(Self::KeyReleased { hwnd, vk_code, modifiers }),
            WM_SYSKEYDOWN => Ok(Self::SystemKeyPressed { hwnd, vk_code, modifiers }),
            WM_SYSKEYUP => Ok(Self::SystemKeyReleased { hwnd, vk_code, modifiers }),
            _ => Err(()),
        }
    }
}