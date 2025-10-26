use crate::platform::agnostic::{events::Event, window::{IWindow, WindowID}};

pub trait WindowHandler {
    fn handle_event(&mut self, event: Event);
    fn get_window(&self) -> &dyn IWindow;
    fn get_window_id(&self) -> WindowID;
}