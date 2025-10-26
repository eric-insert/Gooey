use std::any::Any;

use crate::platform::{agnostic::{events::Event, window::{IWindow, WindowConfig, WindowHandler, WindowID}}, create_platform_window};

pub struct Window {
    pub id: WindowID,
    platform_window: Box<dyn IWindow>, // WindowsWindow, LinuxWindow, etc.
}

impl IWindow for Window {
    fn create(config: &WindowConfig) -> Self {
        let id = WindowID::generate();
        let platform_window = create_platform_window(config); // returns Box<dyn IWindow>
        Self { id, platform_window }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_title(&self, title: &str) {
        self.platform_window.set_title(title);
    }

    fn set_visible(&self, visible: bool) {
        self.platform_window.set_visible(visible);
    }
    
    fn set_resizable(&self, resizable: bool) {
        self.platform_window.set_resizable(resizable);
    }
    
    fn set_decorated(&self, decorated: bool) {
        self.platform_window.set_decorated(decorated);
    }
    
    fn request_redraw(&self) {
        self.platform_window.request_redraw();
    }
    
    fn destroy(&mut self) {
        self.platform_window.destroy();
    }
    
    fn close(&self) {
        self.platform_window.close();
    }
    
    fn set_minimized(&self, minimized: bool) {
        self.platform_window.set_minimized(minimized);
    }
    
    fn set_maximized(&self, maximized: bool) {
        self.platform_window.set_maximized(maximized);
    }
    
    fn set_focus(&self) {
        self.platform_window.set_focus();
    }
    
    fn set_always_visible(&self, always: bool) {
        self.platform_window.set_always_visible(always);
    }
    
    fn set_fullscreen(&self, fullscreen: bool) {
        self.platform_window.set_fullscreen(fullscreen);
    }
    
    fn set_width(&self, width: u32) {
        self.platform_window.set_width(width);
    }
    
    fn set_height(&self, height: u32) {
        self.platform_window.set_height(height);
    }
    
    fn set_size(&self, width: u32, height: u32) {
        self.platform_window.set_size(width, height);
    }
    
    fn set_min_width(&self, width: u32) {
        self.platform_window.set_min_width(width);
    }
    
    fn set_max_width(&self, width: u32) {
        self.platform_window.set_max_width(width);
    }
    
    fn set_min_height(&self, height: u32) {
        self.platform_window.set_min_height(height);
    }
    
    fn set_max_height(&self, height: u32) {
        self.platform_window.set_max_height(height);
    }
    
    fn set_min_size(&self, width: u32, height: u32) {
        self.platform_window.set_min_size(width, height);
    }
    
    fn set_max_size(&self, width: u32, height: u32) {
        self.platform_window.set_max_size(width, height);
    }
    
    fn set_position(&self, x: i32, y: i32) {
        self.platform_window.set_position(x, y);
    }
    
    fn set_anchored(&self, anchor: super::AnchorType) {
        self.platform_window.set_anchored(anchor);
    }
    
    fn set_cursor_visible(&self, visible: bool) {
        self.platform_window.set_cursor_visible(visible);
    }
    
    fn set_cursor_icon(&self, icon: crate::platform::agnostic::accessories::CursorIcon) {
        self.platform_window.set_cursor_icon(icon);
    }
    
    fn set_cursor_locked(&self, locked: bool) {
        self.platform_window.set_cursor_locked(locked);
    }
}

impl WindowHandler for Window {
    fn handle_event(&mut self, event: Event) {
        // later you’ll probably forward this to UI, input handling, etc.
        println!("Window {:?} received event: {:?}", self.id, event);
    }

    fn get_window(&self) -> &dyn IWindow {
        &*self.platform_window
    }

    fn get_window_id(&self) -> WindowID {
        self.id
    }
}

impl Window {
    pub fn id(&self) -> WindowID {
        self.id
    }
}