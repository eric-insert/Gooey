use std::{collections::HashMap};

use crate::platform::agnostic::{events::{Event, PlatformDriver, WindowEvent}, window::{IWindow, WindowID}};

use windows::{core::BOOL, Win32::{UI::WindowsAndMessaging::*}};


pub struct WindowsPlatformDriver {
    hwnd_to_id: HashMap<isize, WindowID>, // isize = HWND as usize
}

impl WindowsPlatformDriver {
    pub fn new() -> Self {
        Self {
            hwnd_to_id: HashMap::new(),
        }
    }

    fn handle_message(&mut self, result: BOOL, msg: MSG, push: &mut dyn FnMut(Event)) {
    match result.0 {
        -1 => {
            eprintln!("GetMessageW error!");
        }
        0 => {
            println!("WM_QUIT received");

            for window_id in self.hwnd_to_id.values() {
                push(Event::WindowEvent {
                    window_id: *window_id,
                    event: WindowEvent::CloseRequested,
                });
            }
        }
        _ => {
                unsafe {
                    if TranslateMessage(&msg).as_bool() {
                        println!("Received Message: {:?}", &msg);
                    }
                    DispatchMessageW(&msg);
                }
            }
        }
    }
}


impl PlatformDriver for WindowsPlatformDriver {
    fn poll_events(&mut self, push: &mut dyn FnMut(Event)) {
        unsafe {
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                self.handle_message(BOOL(0), msg, push); // fake result: BOOL(1) = "got a message"
            }
        }
    }

    fn wait_events(&mut self, push: &mut dyn FnMut(Event)) {
        unsafe {
            let mut msg = MSG::default();
            let result = GetMessageW(&mut msg, None, 0, 0);
            self.handle_message(result, msg, push);
        }
    }

    fn register_window(&mut self, id: WindowID, window: &dyn IWindow) {
        if let Some(win) = window.as_any().downcast_ref::<crate::platform::windows::Window>() {
            let hwnd = win.hwnd.0 as isize;
            self.hwnd_to_id.insert(hwnd, id);
            println!("Registered Window with ID: {:?}", id);
        } else {
            eprintln!("register_window: window is not a WindowsWindow");
        }
    }
}


