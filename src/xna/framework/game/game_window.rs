use std::mem;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::UI::WindowsAndMessaging::{AdjustWindowRectEx, CreateWindowExA, DefWindowProcW, GetSystemMetrics, GetWindowLongA, LoadCursorA, LoadCursorW, LoadIconA, LoadIconW, MoveWindow, PostQuitMessage, RegisterClassExA, CS_DBLCLKS, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, GWL_EXSTYLE, GWL_STYLE, IDC_ARROW, IDI_APPLICATION, SM_CXSCREEN, SM_CYSCREEN, WINDOW_EX_STYLE, WINDOW_STYLE, WNDCLASSEXA, WS_EX_TOPMOST, WS_OVERLAPPED, WS_POPUP, WS_SYSMENU, WS_VISIBLE};
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::{CreateSolidBrush, RGBTRIPLE};
use crate::xna::framework::game::{GameWindow, GameWindowError, GameWindowStyle};
use crate::xna::framework::{Point, Rectangle, Vector2};

impl GameWindow {
    pub fn client_bounds(&self) -> Rectangle {
        Rectangle {
            x: self.window_pos_x,
            y: self.window_pos_y,
            width: self.window_width as i32,
            height: self.window_height as i32
        }
    }

    fn convert_window_style_to_u32(&self) -> u32 {
        match self.window_style {
            GameWindowStyle::Windowed => WS_OVERLAPPED.0 | WS_SYSMENU.0 | WS_VISIBLE.0,
            GameWindowStyle::FullScreen => WS_POPUP.0 | WS_VISIBLE.0,
            GameWindowStyle::BorderlessFullScreen => WS_EX_TOPMOST.0 | WS_POPUP.0 | WS_VISIBLE.0
        }
    }

    fn apply_windowed_config(&mut self, hwnd: &HWND) {
        unsafe{

            let mut win_rect = RECT{ left: 0, top: 0, right: self.window_width, bottom: self.window_height };
            let win_style = GetWindowLongA(*hwnd, GWL_STYLE);
            let win_ex_style = GetWindowLongA(*hwnd, GWL_EXSTYLE);

            let s = WINDOW_STYLE(5);

            let win_style2 = WINDOW_STYLE(win_style as u32);
            let win_ex_style2 = WINDOW_EX_STYLE(win_ex_style as u32);

            AdjustWindowRectEx(&mut win_rect, win_style2, false, win_ex_style2).unwrap();

            let cx_screen = GetSystemMetrics(SM_CXSCREEN);
            let cy_screen = GetSystemMetrics(SM_CYSCREEN);

            self.window_pos_x = (cx_screen / 2) - ((win_rect.right - win_rect.left) / 2);
            self.window_pos_y = (cy_screen / 2) - ((win_rect.bottom - win_rect.top) / 2);

            MoveWindow(
                *hwnd,
                self.window_pos_x,
                self.window_pos_y,
                win_rect.right - win_rect.left,
                win_rect.bottom - win_rect.top,
                true
            ).unwrap();
        }
    }

    pub fn create(&mut self) -> Result<(), GameWindowError> {
        let class_name = "XnaGameWindow";

        let mut wnd_class = WNDCLASSEXA::default();
        wnd_class.cbSize = mem::size_of::<WNDCLASSEXA>() as u32;
        wnd_class.style = CS_DBLCLKS | CS_OWNDC | CS_HREDRAW | CS_VREDRAW;
        wnd_class.lpfnWndProc = Some(Self::wnd_proc);
        wnd_class.cbClsExtra = 0;
        wnd_class.cbWndExtra = 0;
        wnd_class.lpszMenuName = PCSTR::null();
        wnd_class.lpszClassName = PCSTR::from_raw(class_name.as_ptr());

        unsafe {
            let h_module = GetModuleHandleA(None);

            if h_module.is_err() {
                return Err(GameWindowError {
                    message: "Failed to get module handle".to_string(),
                })
            }

            let h_instance = HINSTANCE::from(h_module.unwrap());
            wnd_class.hInstance = h_instance;

            let icon = PCSTR(IDI_APPLICATION.to_string().unwrap().as_ptr());
            let cursor = PCSTR(IDC_ARROW.to_string().unwrap().as_ptr());

            wnd_class.hIcon = LoadIconA(None, icon).unwrap();
            wnd_class.hCursor = LoadCursorA(None, cursor).unwrap();
            wnd_class.hbrBackground = CreateSolidBrush(COLORREF(0));
            wnd_class.hIconSm = LoadIconA(None, icon).unwrap();

            let register_result = RegisterClassExA(&wnd_class);

            if register_result == 0 {
                return Err(GameWindowError{
                    message: "RegisterClass failed".to_string(),
                });
            }

            let style = self.convert_window_style_to_u32();
            let ex_style = WINDOW_EX_STYLE(style);

            let window_handle = CreateWindowExA(
                ex_style,
                PCSTR::from_raw(class_name.as_ptr()),
                PCSTR::from_raw(self.window_title.as_ptr()),
                WINDOW_STYLE(0),
                self.window_pos_x,
                self.window_pos_y,
                self.window_width,
                self.window_height,
                None,
                None,
                Some(h_instance),
                None
            ).unwrap();

            if self.window_style != GameWindowStyle::Windowed {
                self.apply_windowed_config(&window_handle);
            }
        }

        Ok(())
    }

    extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match msg {
                WM_PAINT => {
                    // Aqui poderia ser feito o desenho da janela, se necessário
                    LRESULT(0)
                }
                WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcW(hwnd, msg, wparam, lparam),
            }
        }
    }
}