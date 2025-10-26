use windows::{core::PCWSTR, Win32::{Foundation::LPARAM, UI::{Input::KeyboardAndMouse::{GetKeyState, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT}, WindowsAndMessaging::{CS_DBLCLKS, CS_HREDRAW, CS_NOCLOSE, CS_OWNDC, CS_VREDRAW, WNDCLASS_STYLES}}}};

use crate::platform::agnostic::{input::Modifiers, window::WindowConfig};

pub fn to_wstring(value: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    use std::ffi::OsStr;

    let mut v: Vec<u16> = OsStr::new(value)
        .encode_wide()
        .collect();
    v.push(0); // null terminator
    v
}

pub fn to_pcwstr(value: &str) -> PCWSTR {
    let word = to_wstring(value);
    PCWSTR(word.as_ptr())
}

pub fn to_class_styles(config: &WindowConfig) -> WNDCLASS_STYLES {
    let mut style = WNDCLASS_STYLES(0);

    // Redraw window when resized
    if config.resizable.unwrap_or(true) {
        style |= CS_HREDRAW | CS_VREDRAW;
    }
    // Needed for Vulkan/OpenGL/etc
    if config.gpu_access.unwrap_or(false) {
        style |= CS_OWNDC;
    }

    // Enable double-click detection
    if config.dbl_click_detect.unwrap_or(false) {
        style |= CS_DBLCLKS;
    }

    // Disable close button
    if config.uncloseable.unwrap_or(false) {
        style |= CS_NOCLOSE;
    }

    style
}

pub fn get_x_lparam(lp: LPARAM) -> i32 {
    (lp.0 & 0xFFFF) as i16 as i32
}
pub fn get_y_lparam(lp: LPARAM) -> i32 {
    ((lp.0 >> 16) & 0xFFFF) as i16 as i32
}

pub fn get_modifiers() -> Modifiers {
    let mut mods = Modifiers::empty();

    unsafe {
        if GetKeyState(VK_CONTROL.0 as i32) < 0 {
            mods.insert(Modifiers::CONTROL);
        }
        if GetKeyState(VK_SHIFT.0 as i32) < 0 {
            mods.insert(Modifiers::SHIFT);
        }
        if GetKeyState(VK_MENU.0 as i32) < 0 {
            mods.insert(Modifiers::ALT);
        }
        if GetKeyState(VK_LWIN.0 as i32) < 0 || GetKeyState(VK_RWIN.0 as i32) < 0 {
            mods.insert(Modifiers::SUPER);
        }
    }

    mods
}