use std::{any::Any, sync::Once};

use windows::{core::PCWSTR, Win32::{Foundation::{HINSTANCE, HWND, LPARAM, POINT, RECT, WPARAM}, Graphics::Gdi::{ClientToScreen, InvalidateRect, COLOR_WINDOW, HBRUSH}, System::LibraryLoader::GetModuleHandleW, UI::{Input::KeyboardAndMouse::SetFocus, WindowsAndMessaging::{ClipCursor, CreateWindowExW, DestroyWindow, GetClientRect, GetWindowLongW, LoadCursorW, LoadIconW, PostMessageW, RegisterClassExW, SetCursor, SetWindowLongW, SetWindowPos, SetWindowTextW, ShowCursor, ShowWindow, CW_USEDEFAULT, GWL_STYLE, HWND_NOTOPMOST, HWND_TOPMOST, IDC_ARROW, IDI_APPLICATION, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_RESTORE, SW_SHOW, WM_CLOSE, WNDCLASSEXW, WS_CAPTION, WS_OVERLAPPEDWINDOW, WS_POPUP, WS_SYSMENU, WS_THICKFRAME}}}};

use crate::platform::{agnostic::window::{IWindow, WindowConfig}, windows::{callback::window_proc, utils::{to_class_styles, to_pcwstr}}};

static REGISTER_CLASS: Once = Once::new();

pub struct Window {
    pub hwnd: HWND,
    pub hinstance: HINSTANCE,
    pub class_name: PCWSTR,
}

impl IWindow for Window {
    fn create(config: &WindowConfig) -> Self {
        unsafe {
            // Register a window class
            let class_name = to_pcwstr("GooeyWindow");
            let h_module = GetModuleHandleW(None).unwrap();
            let h_instance: HINSTANCE = h_module.into();
            let color_id = COLOR_WINDOW.0 + 1;
            let brush = HBRUSH(color_id as usize as *mut core::ffi::c_void);
            let wnd_class = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: to_class_styles(config),
                lpfnWndProc: Some(window_proc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: h_instance,
                lpszClassName: class_name,
                hIcon: LoadIconW(None, IDI_APPLICATION).unwrap_or_default(),
                hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
                lpszMenuName: PCWSTR::null(),
                hbrBackground: brush,
                hIconSm: LoadIconW(None, IDI_APPLICATION).unwrap_or_default(),
                ..Default::default()
            };

            REGISTER_CLASS.call_once(|| {
                let atom = RegisterClassExW(&wnd_class);
                assert!(atom != 0, "RegisterClassExW failed");
            });

            let position = config.screen_position.unwrap_or((CW_USEDEFAULT, CW_USEDEFAULT));
            let width = config.width.unwrap_or(800);
            let height = config.height.unwrap_or(600);
            let style = if config.decorations.unwrap_or(true) {
                WS_OVERLAPPEDWINDOW
            } else {
                WS_POPUP // No decorations at all, clean surface
            };
            // Create a dummy window
            let hwnd = CreateWindowExW( 
                Default::default(),
                class_name,
                to_pcwstr(config.title),
                style,
                position.0,
                position.1,
                width,
                height,
                None,
                None,
                Some(h_instance),
                Some(std::ptr::null_mut()),
            ).unwrap();

            if config.visible {
                let _ = ShowWindow(hwnd, SW_SHOW);
            } else {
                let _ = ShowWindow(hwnd, SW_HIDE);
            }

            Self { hwnd, hinstance: h_instance, class_name }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_title(&self, title: &str) {
        unsafe {
            SetWindowTextW(self.hwnd, to_pcwstr(title));
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ShowWindow(self.hwnd, if visible { SW_SHOW } else { SW_HIDE });
        }
    } // assumes you have this

    fn set_resizable(&self, resizable: bool) {
        unsafe {
            let style = GetWindowLongW(self.hwnd, GWL_STYLE);
            let new_style = if resizable {
                style | WS_THICKFRAME.0 as i32
            } else {
                style & !(WS_THICKFRAME.0 as i32)
            };
            SetWindowLongW(self.hwnd, GWL_STYLE, new_style);
            SetWindowPos(self.hwnd, None, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED);
        }
    }

    fn set_decorated(&self, decorated: bool) {
        unsafe {
            let style = GetWindowLongW(self.hwnd, GWL_STYLE);
            let new_style = if decorated {
                style | WS_CAPTION.0 as i32 | WS_SYSMENU.0 as i32
            } else {
                style & !(WS_CAPTION.0 as i32 | WS_SYSMENU.0 as i32)
            };
            SetWindowLongW(self.hwnd, GWL_STYLE, new_style);
            SetWindowPos(self.hwnd, None, 0, 0, 0, 0, SWP_NOMOVE | SWP_NOSIZE | SWP_FRAMECHANGED);
        }
    }

    fn request_redraw(&self) {
        unsafe {
            InvalidateRect(Some(self.hwnd), None, true);
        }
    }

    fn destroy(&mut self) {
        unsafe {
            DestroyWindow(self.hwnd).ok();
        }
    }

    fn close(&self) {
        unsafe {
            PostMessageW(Some(self.hwnd), WM_CLOSE, WPARAM(0), LPARAM(0));
        }
    }

    fn set_minimized(&self, minimized: bool) {
        unsafe {
            ShowWindow(self.hwnd, if minimized { SW_MINIMIZE } else { SW_RESTORE });
        }
    }

    fn set_maximized(&self, maximized: bool) {
        unsafe {
            ShowWindow(self.hwnd, if maximized { SW_MAXIMIZE } else { SW_RESTORE });
        }
    }

    fn set_focus(&self) {
        unsafe {
            SetFocus(Some(self.hwnd));
        }
    }

    fn set_always_visible(&self, always: bool) {
        unsafe {
            SetWindowPos(
                self.hwnd,
                if always { Some(HWND_TOPMOST) } else { Some(HWND_NOTOPMOST) },
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE
            );
        }
    }

    fn set_fullscreen(&self, fullscreen: bool) {
        // Simplified; real fullscreen should save/restore style and size
        self.set_resizable(fullscreen);
        self.set_decorated(!fullscreen);
        unsafe {
            ShowWindow(self.hwnd, if fullscreen { SW_MAXIMIZE } else { SW_RESTORE });
        }
    }

    fn set_width(&self, width: u32) {
        let (_, h) = self.get_size();
        self.set_size(width, h);
    }

    fn set_height(&self, height: u32) {
        let (w, _) = self.get_size();
        self.set_size(w, height);
    }

    fn set_size(&self, width: u32, height: u32) {
        unsafe {
            SetWindowPos(self.hwnd, None, 0, 0, width as i32, height as i32, SWP_NOMOVE);
        }
    }

    fn set_min_width(&self, _width: u32) {}
    fn set_max_width(&self, _width: u32) {}
    fn set_min_height(&self, _height: u32) {}
    fn set_max_height(&self, _height: u32) {}
    fn set_min_size(&self, _w: u32, _h: u32) {}
    fn set_max_size(&self, _w: u32, _h: u32) {}

    fn set_position(&self, x: i32, y: i32) {
        unsafe {
            SetWindowPos(self.hwnd, None, x, y, 0, 0, SWP_NOSIZE);
        }
    }

    fn set_anchored(&self, _anchor: crate::platform::agnostic::window::AnchorType) {
        // unimplemented; this is layout behavior
    }

    fn set_cursor_visible(&self, visible: bool) {
        unsafe {
            ShowCursor(visible);
        }
    }

    fn set_cursor_icon(&self, icon: crate::platform::agnostic::accessories::CursorIcon) {
        let win_cursor = icon.to_windows_cursor();
        unsafe { SetCursor(Some(win_cursor)) };
    }

    fn set_cursor_locked(&self, locked: bool) {
        unsafe {
            if locked {
                let mut rect = Default::default();
                GetClientRect(self.hwnd, &mut rect);
                let mut pt = POINT { x: 0, y: 0 };
                ClientToScreen(self.hwnd, &mut pt);
                rect.left += pt.x;
                rect.top += pt.y;
                rect.right += pt.x;
                rect.bottom += pt.y;
                ClipCursor(Some(&rect));
            } else {
                ClipCursor(None);
            }
        }
    }
}

impl Window {
    fn get_size(&self) -> (u32, u32) {
        let mut rect = RECT::default();
        unsafe {
            GetClientRect(self.hwnd, &mut rect);
        }
        (
            (rect.right - rect.left) as u32,
            (rect.bottom - rect.top) as u32,
        )
    }
}