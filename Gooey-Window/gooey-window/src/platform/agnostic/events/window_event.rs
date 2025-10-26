#[derive(Debug, Clone)]
pub enum WindowEvent {
    CloseRequested,
    Destroyed,
    Resized { width: u32, height: u32 },
    Moved { x: i32, y: i32 },
    RedrawRequested,
    FocusGained,
    FocusLost,
    Minimized,
    Restored,
    ScaleFactorChanged,
}