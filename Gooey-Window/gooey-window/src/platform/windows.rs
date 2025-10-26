pub mod window;
pub mod windows_platform_driver;
pub mod callback;
pub mod utils;
pub mod events;

pub use windows_platform_driver::WindowsPlatformDriver as DefaultPlatformDriver;

use crate::platform::{agnostic::window::{IWindow, WindowConfig}, windows::window::Window};


pub fn create_platform_window(config: &WindowConfig) -> Box<dyn IWindow> {
    Box::new(Window::create(config))
}