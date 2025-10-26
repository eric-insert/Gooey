use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        const SHIFT   = 0b0000_0001;
        const CONTROL = 0b0000_0010;
        const ALT     = 0b0000_0100;
        const SUPER   = 0b0000_1000;
    }
}

impl Default for Modifiers {
    fn default() -> Self {
        Self::empty()
    }
}

impl Modifiers {
    pub fn none() -> Self {
        Self::empty()
    }

    pub fn is_ctrl(self) -> bool {
        self.contains(Self::CONTROL)
    }

    pub fn is_shift(self) -> bool {
        self.contains(Self::SHIFT)
    }

    pub fn is_alt(self) -> bool {
        self.contains(Self::ALT)
    }

    pub fn is_super(self) -> bool {
        self.contains(Self::SUPER)
    }

    pub fn any(self) -> bool {
        !self.is_empty()
    }
}