use crate::platform::agnostic::{state::InputState, window::window_id::WindowID};

use super::{
    FileEvent,
    KeyboardEvent,
    MouseEvent,
    WindowEvent,
};

#[derive(Debug, Clone)]
pub enum Event {
    WindowEvent { window_id: WindowID, event: WindowEvent },
    KeyboardEvent { window_id: WindowID, event: KeyboardEvent },
    MouseEvent { window_id: WindowID, event: MouseEvent },
    FileEvent { window_id: WindowID, event: FileEvent },
}

impl Event {
    pub fn update_input_state(&self, state: &mut InputState) {
        match self {
            Event::KeyboardEvent { event, .. } => event.update_input_state(state),
            Event::MouseEvent { event, .. } => event.update_input_state(state),
            _ => {}
        }
    }

    pub fn window_id(&self) -> Option<WindowID> {
        match self {
            Event::KeyboardEvent { window_id, .. }
            | Event::MouseEvent { window_id, .. }
            | Event::WindowEvent { window_id, .. }
            | Event::FileEvent { window_id, .. } => Some(*window_id),
        }
    }
}



