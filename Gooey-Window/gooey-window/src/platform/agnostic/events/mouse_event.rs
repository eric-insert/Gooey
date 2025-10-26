use crate::platform::agnostic::{input::{Modifiers, MouseButton}, state::InputState};

#[derive(Debug, Clone)]
pub enum MouseEvent {
    MouseMoved { x: i32, y: i32, modifiers: Modifiers },
    MouseEntered,
    MouseLeft,
    MouseButtonPressed { button: MouseButton, modifiers: Modifiers },
    MouseButtonReleased { button: MouseButton, modifiers: Modifiers },
    MouseWheelScrolled { delta: i32, modifiers: Modifiers },
}

impl MouseEvent {
    pub fn update_input_state(&self, state: &mut InputState) {
        match self {
            MouseEvent::MouseButtonPressed { button, .. } => {
                state.pressed_mouse_buttons.push(*button);
            }
            MouseEvent::MouseButtonReleased { button, .. } => {
                state.pressed_mouse_buttons.retain(|b| b != button);
            }
            MouseEvent::MouseMoved { x, y, .. } => {
                state.mouse_position = Some((*x, *y));
            }
            MouseEvent::MouseEntered => {
                state.cursor_in_window = true;
            }
            MouseEvent::MouseLeft => {
                state.cursor_in_window = false;
            }
            _ => {}
        }
    }
}
