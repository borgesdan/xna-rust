use crate::xna::csharp::forms::Screen;
use crate::xna::csharp::Exception;
use crate::xna::framework::game::{GameWindow, GameWindowStyle};
use std::fmt::Pointer;
use std::mem;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::CreateSolidBrush;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::{AdjustWindowRectEx, CreateWindowExW, DefWindowProcW, GetSystemMetrics, GetWindowLongA, LoadCursorW, LoadIconW, MoveWindow, PostMessageA, PostQuitMessage, RegisterClassExW, CS_DBLCLKS, CS_HREDRAW, CS_OWNDC, CS_VREDRAW, GWL_EXSTYLE, GWL_STYLE, IDC_ARROW, IDI_APPLICATION, SM_CXSCREEN, SM_CYSCREEN, WINDOW_EX_STYLE, WINDOW_STYLE, WM_DESTROY, WM_PAINT, WNDCLASSEXW, WS_EX_TOPMOST, WS_OVERLAPPED, WS_POPUP, WS_SYSMENU, WS_VISIBLE};

impl GameWindow {
    pub fn close(&self) -> Result<(), Exception> {
        unsafe {
            let post = PostMessageA(Some(self.platform.hwnd), WM_DESTROY, WPARAM(0), LPARAM(0));

            match post {
                Ok(_) => {Ok(())}
                Err(_) => {Err(Exception::new("PostMessageA() failed", None))}
            }
        }
    }

    pub fn screen_from_handle(hwnd: HWND) -> Result<Screen, Exception> {
        //TODO

        Ok(Screen::default())
    }

    pub fn scree_device_name(&self) -> Result<String, Exception> {
        let screen = Self::screen_from_handle(self.platform.hwnd).unwrap();
        //let name = screen.device_name();
        //TODO: implementar screen_device_name
        let name = String::new();
        Ok(name)
    }

    pub fn create(&mut self) -> Result<(), Exception> {
        unsafe {
            let class_name = Self::to_wide("XnaGameWindow");
            let h_module = GetModuleHandleW(None).unwrap();
            let h_instance = HINSTANCE::from(h_module);

            let wnd_class = WNDCLASSEXW {
                cbSize : mem::size_of::<WNDCLASSEXW>() as u32,
                style : CS_DBLCLKS | CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
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

            let style = Self::convert_window_style_to_u32(&self.style);
            let ex_style = WINDOW_EX_STYLE(style);
            let wn_style = WINDOW_STYLE(style);

            let window_handle = CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR(Self::to_wide(self.title.as_str()).as_ptr()),
                wn_style,
                0,
                0,
                self.width,
                self.height,
                None,
                None,
                Some(h_instance),
                None,
            ).unwrap();

            if self.style == GameWindowStyle::Windowed {
                Self::apply_windowed(&window_handle, self);
            }

            Ok(())
        }
    }

    fn apply_windowed(hwnd: &HWND, game_window: &mut GameWindow) {
        unsafe {
            let mut win_rect = RECT { left: 0, top: 0, right: game_window.width, bottom: game_window.height };
            let win_style = GetWindowLongA(*hwnd, GWL_STYLE);
            let win_ex_style = GetWindowLongA(*hwnd, GWL_EXSTYLE);

            let win_style2 = WINDOW_STYLE(win_style as u32);
            let win_ex_style2 = WINDOW_EX_STYLE(win_ex_style as u32);

            AdjustWindowRectEx(&mut win_rect, win_style2, false, win_ex_style2).unwrap();

            let cx_screen = GetSystemMetrics(SM_CXSCREEN);
            let cy_screen = GetSystemMetrics(SM_CYSCREEN);

            game_window.x = (cx_screen / 2) - ((win_rect.right - win_rect.left) / 2);
            game_window.y = (cy_screen / 2) - ((win_rect.bottom - win_rect.top) / 2);

            MoveWindow(
                *hwnd,
                game_window.x,
                game_window.y,
                win_rect.right - win_rect.left,
                win_rect.bottom - win_rect.top,
                true,
            ).unwrap();
        }
    }

    fn convert_window_style_to_u32(style: &GameWindowStyle) -> u32 {
        match style {
            GameWindowStyle::Windowed => WS_OVERLAPPED.0 | WS_SYSMENU.0 | WS_VISIBLE.0,
            GameWindowStyle::FullScreen => WS_POPUP.0 | WS_VISIBLE.0,
            GameWindowStyle::BorderlessFullScreen => WS_EX_TOPMOST.0 | WS_POPUP.0 | WS_VISIBLE.0
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