use std::mem;
use std::fmt::Pointer;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LRESULT, WPARAM, LPARAM, HINSTANCE, RECT, COLORREF};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage, RegisterClassExW, ShowWindow, TranslateMessage, MSG, WNDCLASSEXW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, SW_SHOW, WM_DESTROY, WM_PAINT, WM_QUIT, WS_OVERLAPPEDWINDOW, WINDOW_EX_STYLE, WS_OVERLAPPED, WS_SYSMENU, WS_VISIBLE, WS_POPUP, WS_EX_TOPMOST, GetWindowLongA, GWL_STYLE, GWL_EXSTYLE, WINDOW_STYLE, AdjustWindowRectEx, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN, MoveWindow, LoadIconW, IDI_APPLICATION};
use crate::xna::framework::game::{GameWindow, GameWindowError, GameWindowStyle};
use crate::xna::framework::{Point, Rectangle, Vector2};

impl GameWindow {
    pub fn client_bounds(&self) -> Rectangle {
        Rectangle {
            x: self.window_pos_x,
            y: self.window_pos_y,
            width: self.window_width as i32,
            height: self.window_height as i32,
        }
    }

    fn convert_window_style_to_u32(style: &GameWindowStyle) -> u32 {
        match style {
            GameWindowStyle::Windowed => WS_OVERLAPPED.0 | WS_SYSMENU.0 | WS_VISIBLE.0,
            GameWindowStyle::FullScreen => WS_POPUP.0 | WS_VISIBLE.0,
            GameWindowStyle::BorderlessFullScreen => WS_EX_TOPMOST.0 | WS_POPUP.0 | WS_VISIBLE.0
        }
    }

    fn apply_windowed(hwnd: &HWND, game_window: &mut GameWindow) {
        unsafe {
            let mut win_rect = RECT { left: 0, top: 0, right: game_window.window_width, bottom: game_window.window_height };
            let win_style = GetWindowLongA(*hwnd, GWL_STYLE);
            let win_ex_style = GetWindowLongA(*hwnd, GWL_EXSTYLE);

            let s = WINDOW_STYLE(5);

            let win_style2 = WINDOW_STYLE(win_style as u32);
            let win_ex_style2 = WINDOW_EX_STYLE(win_ex_style as u32);

            AdjustWindowRectEx(&mut win_rect, win_style2, false, win_ex_style2).unwrap();

            let cx_screen = GetSystemMetrics(SM_CXSCREEN);
            let cy_screen = GetSystemMetrics(SM_CYSCREEN);

            game_window.window_pos_x = (cx_screen / 2) - ((win_rect.right - win_rect.left) / 2);
            game_window.window_pos_y = (cy_screen / 2) - ((win_rect.bottom - win_rect.top) / 2);

            MoveWindow(
                *hwnd,
                game_window.window_pos_x,
                game_window.window_pos_y,
                win_rect.right - win_rect.left,
                win_rect.bottom - win_rect.top,
                true,
            ).unwrap();
        }
    }

    pub fn create_window(window_size: Point, window_style: GameWindowStyle, window_title: &str) -> Result<GameWindow, GameWindowError> {
        unsafe {
            let class_name = Self::to_wide("XnaGameWindow");
            let h_module = GetModuleHandleW(None).unwrap();
            let h_instance = HINSTANCE::from(h_module);

            let wnd_class = WNDCLASSEXW {
                cbSize : mem::size_of::<WNDCLASSEXW>() as u32,
                style : CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc : Some(Self::wnd_proc),
                lpszClassName : PCWSTR(class_name.as_ptr()),
                hInstance : h_instance.into(),
                hIcon : LoadIconW(None, IDI_APPLICATION).unwrap(),
                hCursor : LoadCursorW(None, IDC_ARROW).unwrap(),
                hbrBackground : CreateSolidBrush(COLORREF(0)),
                hIconSm : LoadIconW(None, IDI_APPLICATION).unwrap(),
                ..Default::default()
            };

            RegisterClassExW(&wnd_class);

            let style = Self::convert_window_style_to_u32(&window_style);
            let ex_style = WINDOW_EX_STYLE(style);
            let wn_style = WINDOW_STYLE(style);

            let window_handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR(Self::to_wide(window_title).as_ptr()),
                wn_style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                window_size.x,
                window_size.y,
                None,
                None,
                Some(h_instance),
                None,
            ).unwrap();

            let mut new_window = GameWindow::default();
            new_window.window_style = GameWindowStyle::from(window_style);
            new_window.window_width = window_size.x;
            new_window.window_height = window_size.y;
            new_window.window_pos_x = 0;
            new_window.window_pos_y = 0;

            if new_window.window_style == GameWindowStyle::Windowed {
                Self::apply_windowed(&window_handle, &mut new_window);
            }

            Ok(new_window)
        }
    }

    fn to_wide(s: &str) -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    }

    pub extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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