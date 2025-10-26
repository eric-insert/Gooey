use windows::Win32::Foundation::HWND;

#[derive(Debug)]
pub enum Win32WindowEvent {
    Created { hwnd: HWND },
    CloseRequested { hwnd: HWND },
    Destroyed { hwnd: HWND },
    Shown { hwnd: HWND },
    Hidden { hwnd: HWND },
    Minimized { hwnd: HWND },
    Maximized { hwnd: HWND },
    Restored { hwnd: HWND },
    FocusGained { hwnd: HWND },
    FocusLost { hwnd: HWND },
    Moved { hwnd: HWND, x: i32, y: i32 },
    Resized { hwnd: HWND, width: i32, height: i32 },
    RedrawRequested { hwnd: HWND },
    Paint { hwnd: HWND },
    DPIChanged { hwnd: HWND, dpi: u32 },
    ThemeChanged { hwnd: HWND },
    Activated { hwnd: HWND },
    EnterSizeMove { hwnd: HWND },
    ExitSizeMove { hwnd: HWND },
    Moving { hwnd: HWND, x: i32, y: i32 },
    Sizing { hwnd: HWND, width: i32, height: i32 },
    StyleChanged { hwnd: HWND, new_style: u32 },
    PositionChanged { hwnd: HWND },
    ClearBackground { hwnd: HWND },
}