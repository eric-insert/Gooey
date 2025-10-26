use crate::platform::agnostic::input::Modifiers;

#[derive(Debug, Clone)]
pub enum FileEvent {
    Hovered { path: String, modifiers: Modifiers },
    Dropped { path: String, modifiers: Modifiers },
    Cancelled,
}