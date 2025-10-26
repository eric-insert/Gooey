#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawWindowHandle {
    #[cfg(target_os = "windows")]
    Win32 { hwnd: *mut core::ffi::c_void },

    #[cfg(target_os = "linux")]
    Xlib { window: u64, display: *mut core::ffi::c_void },

    #[cfg(target_os = "macos")]
    AppKit { ns_window: *mut core::ffi::c_void },

    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawDisplayHandle {
    #[cfg(target_os = "windows")]
    Win32 { hinstance: *mut core::ffi::c_void },

    #[cfg(target_os = "linux")]
    Xlib { display: *mut core::ffi::c_void },

    #[cfg(target_os = "macos")]
    AppKit,

    Unknown,
}

pub struct WindowHandle {
    pub window: RawWindowHandle,
    pub display: RawDisplayHandle,
}