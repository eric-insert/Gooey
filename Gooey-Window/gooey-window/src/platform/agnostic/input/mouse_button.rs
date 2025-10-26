#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseButton(pub u8);

impl MouseButton {
    pub const LEFT: Self = MouseButton(0);
    pub const RIGHT: Self = MouseButton(1);
    pub const MIDDLE: Self = MouseButton(2);
    pub const X1: Self = MouseButton(3);
    pub const X2: Self = MouseButton(4);

    pub fn raw(&self) -> u8 {
        self.0
    }
}