use bitvec::{bitvec, order::Lsb0, vec::BitVec};

use crate::platform::agnostic::input::{KeyCode, Modifiers, MouseButton};

#[derive(Debug, Default)]
pub struct InputState {
    key_state: BitVec<u8, Lsb0>,
    pub pressed_mouse_buttons: Vec<MouseButton>,
    pub modifiers: Modifiers,
    pub mouse_position: Option<(i32, i32)>,
    pub cursor_in_window: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            key_state: bitvec![u8, Lsb0; 0; KeyCode::COUNT],
            pressed_mouse_buttons: Vec::new(),
            modifiers: Modifiers::empty(),
            mouse_position: None,
            cursor_in_window: false,
        }
    }

    pub fn set_key_down(&mut self, key: KeyCode, down: bool) {
        if let Some(index) = key.to_index() {
            self.key_state.set(index, down);
        }
    }

    pub fn is_key_down(&self, key: KeyCode) -> bool {
        key.to_index().map_or(false, |i| self.key_state[i])
    }

    pub fn is_mouse_button_down(&self, button: MouseButton) -> bool {
        self.pressed_mouse_buttons.contains(&button)
    }

    pub fn is_ctrl(&self) -> bool {
        self.modifiers.is_ctrl()
    }

    pub fn is_shift(&self) -> bool {
        self.modifiers.is_shift()
    }

    pub fn is_alt(&self) -> bool {
        self.modifiers.is_alt()
    }

    pub fn is_super(&self) -> bool {
        self.modifiers.is_super()
    }

    pub fn update_modifiers(&mut self) {
        let mut mods = Modifiers::empty();
        if self.is_key_down(KeyCode::ShiftLeft) || self.is_key_down(KeyCode::ShiftRight) {
            mods |= Modifiers::SHIFT;
        }
        if self.is_key_down(KeyCode::ControlLeft) || self.is_key_down(KeyCode::ControlRight) {
            mods |= Modifiers::CONTROL;
        }
        if self.is_key_down(KeyCode::AltLeft) || self.is_key_down(KeyCode::AltRight) {
            mods |= Modifiers::ALT;
        }
        if self.is_key_down(KeyCode::SuperLeft) || self.is_key_down(KeyCode::SuperRight) {
            mods |= Modifiers::SUPER;
        }
        self.modifiers = mods;
    }
}