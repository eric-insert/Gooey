use windows::Win32::Foundation::HWND;

#[derive(Debug)]
pub enum Win32IMEEvent {
    StartComposition { hwnd: HWND },
    Composition { hwnd: HWND, composing_text: Option<String>, result_text: Option<String>, cursor_pos: Option<usize>, attributes: Option<Vec<u8>> },
    EndComposition { hwnd: HWND },
    SetContext { hwnd: HWND, enabled: bool },
    Notify { hwnd: HWND, command: u32, data: u32 },
    Request { hwnd: HWND, request_type: u32, data: usize },
    KeyDown { hwnd: HWND, vk_code: u32 },
    KeyUp { hwnd: HWND, vk_code: u32 },
}