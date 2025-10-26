use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
};

const HANDLED: isize = 0;
const UNHANDLED: isize = 1;

pub extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            // You'd normally get a pointer to EventQueue here
            // For now, just testing flow
            println!("WM_CLOSE received");
            unsafe { 
                 if DestroyWindow(hwnd).is_ok() {
                    println!("Window destroyed.");
                    return LRESULT(HANDLED);
                } else {
                    eprintln!("DestroyWindow failed");
                    return LRESULT(UNHANDLED);
                }
            };
        }

        WM_DESTROY => {
            println!("WM_DESTROY");
            LRESULT(HANDLED)
        }

        WM_KEYDOWN => {
            let vk = wparam.0 as u32;
            println!("Key down: {}", vk);
            // Here you'd map vk → KeyCode and push Event::Keyboard
            LRESULT(HANDLED)
        }

        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}