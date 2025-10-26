use std::{ffi::{c_void, OsString}, os::windows::ffi::OsStringExt};

use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::{MSG, WM_DROPFILES}};
use windows::Win32::UI::Shell::{
    DragQueryFileW, DragFinish, HDROP,
};

#[derive(Debug)]
pub enum Win32FileEvent {
    FileDropped { hwnd: HWND, paths: Vec<String> },
}

impl TryFrom<MSG> for Win32FileEvent {
    type Error = ();

    fn try_from(msg: MSG) -> Result<Self, Self::Error> {
        if msg.message != WM_DROPFILES {
            return Err(());
        }

        let hdrop = HDROP(msg.wParam.0 as *mut c_void);
        let hwnd = msg.hwnd;

        unsafe {
            let count = DragQueryFileW(hdrop, 0xFFFFFFFF, None) as usize;
            let mut paths = Vec::with_capacity(count);

            for i in 0..count {
                let len = DragQueryFileW(hdrop, i as u32, None) as usize;
                let mut buffer = vec![0u16; len + 1];
                if DragQueryFileW(hdrop, i as u32, Some(&mut buffer)) > 0 {
                    let os_string = OsString::from_wide(&buffer[..len]);
                    if let Ok(path) = os_string.into_string() {
                        paths.push(path);
                    }
                }
            }

            DragFinish(hdrop);
            Ok(Self::FileDropped { hwnd, paths })
        }
    }
}