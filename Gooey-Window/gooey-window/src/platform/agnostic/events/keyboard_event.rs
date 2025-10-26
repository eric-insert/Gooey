use crate::platform::agnostic::{input::{KeyCode, Modifiers}, state::InputState};

#[derive(Debug, Clone)]
pub enum KeyboardEvent {
    KeyPressed { key_code: KeyCode, repeat: bool, modifiers: Modifiers },
    KeyReleased { key_code: KeyCode, modifiers: Modifiers },
    CharReceived { character: char, modifiers: Modifiers },
}

impl KeyboardEvent {
    pub fn update_input_state(&self, state: &mut InputState) {
        match self {
            KeyboardEvent::KeyPressed { key_code, .. } => {
                state.set_key_down(*key_code, true);
                state.update_modifiers();
            }
            KeyboardEvent::KeyReleased { key_code, .. } => {
                state.set_key_down(*key_code, false);
                state.update_modifiers();
            }
            _ => {}
        }
    }
}