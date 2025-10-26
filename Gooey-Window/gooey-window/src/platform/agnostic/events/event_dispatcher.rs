use std::collections::HashMap;

use crate::platform::agnostic::{events::{Event, EventDispatcherConfig, EventQueue, PlatformDriver}, state::InputState, window::{Window, WindowHandler, WindowID}};

#[cfg(target_os = "windows")]
use crate::platform::windows::DefaultPlatformDriver;

pub struct EventDispatcher {
    pub input_state: InputState,
    pub event_queue: EventQueue,
    pub driver: Box<dyn PlatformDriver>,
    pub default_config: EventDispatcherConfig,
    pub configs: HashMap<WindowID, EventDispatcherConfig>,
    pub windows: HashMap<WindowID, Box<dyn WindowHandler>>,
}

impl EventDispatcher {
    /// Creates a new event loop instance.
    pub fn new(default_config: Option<EventDispatcherConfig>) -> Self {
        Self {
            input_state: InputState::new(),
            event_queue: EventQueue::new(),
            driver: Box::new(DefaultPlatformDriver::new()),
            default_config: default_config.unwrap_or_default(),
            configs: HashMap::new(),
            windows: HashMap::new(),
        }
    }

    pub fn register_window(
        &mut self,
        window: Window,
        config: Option<EventDispatcherConfig>,
    ) {
        let id = window.id();
        let handler: Box<dyn WindowHandler> = Box::new(window);

        self.configs.insert(id, config.unwrap_or_default());
        self.windows.insert(id, handler);

        let handler = self.windows.get(&id).unwrap();
        self.driver.register_window(id, handler.get_window());
    }

    pub fn poll_events<F: FnMut(Event)>(&mut self, mut callback: F) {
        self.driver.poll_events(&mut |event| {
            let dispatchable = if let Some(window_id) = event.window_id() {
                let cfg = self.configs.get(&window_id).unwrap_or(&self.default_config);
                match event {
                    Event::KeyboardEvent { .. } => cfg.allow_keyboard_events,
                    Event::MouseEvent { .. } => cfg.allow_mouse_events,
                    Event::WindowEvent { .. } => cfg.allow_window_events,
                    Event::FileEvent { .. } => cfg.allow_file_events,
                }
            } else {
                false
            };

            if dispatchable {
                event.update_input_state(&mut self.input_state);
                self.event_queue.push(event.clone());

                if let Some(window_id) = event.window_id() {
                    if let Some(handler) = self.windows.get_mut(&window_id) {
                        handler.handle_event(event.clone());
                    }
                }
                callback(event);
            }
        });
    }

    /// Blocking event wait (ideal for GUI apps)
    pub fn wait_events<F: FnMut(Event)>(&mut self, mut callback: F) {
        self.driver.wait_events(&mut |event| {
            let dispatchable = if let Some(window_id) = event.window_id() {
                let cfg = self.configs.get(&window_id).unwrap_or(&self.default_config);
                match event {
                    Event::KeyboardEvent { .. } => cfg.allow_keyboard_events,
                    Event::MouseEvent { .. } => cfg.allow_mouse_events,
                    Event::WindowEvent { .. } => cfg.allow_window_events,
                    Event::FileEvent { .. } => cfg.allow_file_events,
                }
            } else {
                false
            };

            if dispatchable {
                event.update_input_state(&mut self.input_state);
                self.event_queue.push(event.clone());

                if let Some(window_id) = event.window_id() {
                    if let Some(handler) = self.windows.get_mut(&window_id) {
                        handler.handle_event(event.clone());
                    }
                }

                callback(event);
            }
        });
    }

    pub fn push_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    /// Simple internal event loop — GUI-style usage
    pub fn run<F: FnMut(Event)>(&mut self, mut callback: F) {
        loop {
            self.wait_events(|event| callback(event));
        }
    }
}
