use windows::Win32::{Foundation::HINSTANCE, UI::WindowsAndMessaging::{LoadCursorW, HCURSOR, IDC_ARROW, IDC_CROSS, IDC_HAND, IDC_IBEAM, IDC_NO, IDC_SIZEALL, IDC_SIZENS, IDC_SIZEWE, IDC_WAIT}};

#[derive(Debug, Clone)]
pub enum CursorIcon {
    Default,
    Pointer,
    Text,
    ResizeHorizontal,
    ResizeVertical,
    Wait,
    Crosshair,
    Grab,
    Grabbing,
    NotAllowed,
}

impl CursorIcon {
    pub fn to_windows_cursor(self) -> HCURSOR {
        unsafe {
            match self {
                CursorIcon::Default => LoadCursorW(Some(HINSTANCE::default()), IDC_ARROW).unwrap(),
                CursorIcon::Pointer => LoadCursorW(Some(HINSTANCE::default()), IDC_HAND).unwrap(),
                CursorIcon::Text => LoadCursorW(Some(HINSTANCE::default()), IDC_IBEAM).unwrap(),
                CursorIcon::Crosshair => LoadCursorW(Some(HINSTANCE::default()), IDC_CROSS).unwrap(),
                CursorIcon::NotAllowed => LoadCursorW(Some(HINSTANCE::default()), IDC_NO).unwrap(),
                CursorIcon::ResizeHorizontal => LoadCursorW(Some(HINSTANCE::default()), IDC_SIZEWE).unwrap(),
                CursorIcon::ResizeVertical => LoadCursorW(Some(HINSTANCE::default()), IDC_SIZENS).unwrap(),
                CursorIcon::Wait => LoadCursorW(Some(HINSTANCE::default()), IDC_WAIT).unwrap(),
                CursorIcon::Grab => LoadCursorW(Some(HINSTANCE::default()), IDC_HAND).unwrap(), // Close enough
                CursorIcon::Grabbing => LoadCursorW(Some(HINSTANCE::default()), IDC_SIZEALL).unwrap(), // Best approximation
            }
        }
    }
}