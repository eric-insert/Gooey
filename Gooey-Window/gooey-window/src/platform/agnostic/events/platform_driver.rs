use crate::platform::agnostic::{events::Event, window::{IWindow, WindowID}};

pub trait PlatformDriver {
    fn poll_events(&mut self, push: &mut dyn FnMut(Event));
    fn wait_events(&mut self, push: &mut dyn FnMut(Event));
    fn register_window(&mut self, id: WindowID, window: &dyn IWindow);
}