use std::any::Any;

use crate::platform::agnostic::{accessories::CursorIcon, window::{AnchorType, WindowConfig}};

pub trait IWindow {
    // Basic
    fn set_title(&self, title: &str);
    fn set_visible(&self, visible: bool);
    fn set_resizable(&self, resizable: bool);
    fn set_decorated(&self, decorated: bool);
    fn request_redraw(&self);

    // Lifecycle
    fn destroy(&mut self);
    fn close(&self);
    fn create(config: &WindowConfig) -> Self where Self: Sized;
    fn as_any(&self) -> &dyn Any;

    // State
    fn set_minimized(&self, minimized: bool);
    fn set_maximized(&self, maximized: bool);
    fn set_focus(&self);
    fn set_always_visible(&self, always: bool);

    // Fullscreen + size
    fn set_fullscreen(&self, fullscreen: bool);
    fn set_width(&self, width: u32);
    fn set_height(&self, height: u32);
    fn set_size(&self, width: u32, height: u32);
    fn set_min_width(&self, width: u32);
    fn set_max_width(&self, width: u32);
    fn set_min_height(&self, height: u32);
    fn set_max_height(&self, height: u32);
    fn set_min_size(&self, width: u32, height: u32);
    fn set_max_size(&self, width: u32, height: u32);

    // Positioning
    fn set_position(&self, x: i32, y: i32);
    fn set_anchored(&self, anchor: AnchorType);

    // Cursor
    fn set_cursor_visible(&self, visible: bool);
    fn set_cursor_icon(&self, icon: CursorIcon);
    fn set_cursor_locked(&self, locked: bool);
}