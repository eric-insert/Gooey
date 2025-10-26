pub struct EventDispatcherConfig {
    pub allow_keyboard_events: bool,
    pub allow_mouse_events: bool,
    pub allow_window_events: bool,
    pub allow_file_events: bool,
}

impl Default for EventDispatcherConfig {
    fn default() -> Self {
        Self {
            allow_keyboard_events: true,
            allow_mouse_events: true,
            allow_window_events: true,
            allow_file_events: true,
        }
    }
}