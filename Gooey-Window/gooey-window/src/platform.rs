#[cfg(target_os = "windows")]
pub mod windows;

pub mod agnostic;

#[cfg(target_os = "windows")]
pub use self::windows::create_platform_window;

#[cfg(target_os = "linux")]
pub use self::linux::create_platform_window;

#[cfg(target_os = "macos")]
pub use self::macos::create_platform_window;