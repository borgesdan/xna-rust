mod xna;

use std::fmt::Pointer;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LRESULT, WPARAM, LPARAM, GetHandleInformation};
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetModuleHandleW};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage, RegisterClassExW, ShowWindow, TranslateMessage, MSG, WNDCLASSEXW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, SW_SHOW, WM_DESTROY, WM_PAINT, WM_QUIT, WS_OVERLAPPEDWINDOW, WINDOW_EX_STYLE};

use crate::xna::framework::Color;
use crate::xna::csharp::forms::Screen;
use crate::xna::framework::graphics::GraphicsAdapter;

fn main() {

    let mut adapter : Vec<GraphicsAdapter> = Vec::new();

    // unsafe {
    //     // Nome da classe da janela
    //     let class_name = to_wide("MinhaJanelaClass");
    //
    //     // Obter instância do programa
    //     let h_instance = GetModuleHandleW(None).unwrap();
    //
    //     // Definir classe da janela
    //     let wnd_class = WNDCLASSEXW {
    //         cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
    //         style: CS_HREDRAW | CS_VREDRAW,
    //         lpfnWndProc: Some(wnd_proc), // Ponteiro para o callback de eventos da janela
    //         hInstance: h_instance.into(),
    //         lpszClassName: PCWSTR(class_name.as_ptr()),
    //         hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
    //         hbrBackground: HBRUSH(16), // Cor de fundo padrão
    //         ..Default::default()
    //     };
    //
    //     // Registrar a classe da janela
    //     RegisterClassExW(&wnd_class);
    //
    //     // Criar a janela
    //     let hwnd = CreateWindowExW(
    //         WINDOW_EX_STYLE::default(),
    //         PCWSTR(class_name.as_ptr()),
    //         PCWSTR(to_wide("Minha Primeira Janela!").as_ptr()),
    //         WS_OVERLAPPEDWINDOW,
    //         CW_USEDEFAULT,
    //         CW_USEDEFAULT,
    //         800, // Largura
    //         600, // Altura
    //         None,
    //         None,
    //         h_instance,
    //         None,
    //     );
    //
    //     // Exibir a janela
    //     ShowWindow(hwnd, SW_SHOW);
    //
    //     // Loop de mensagens
    //     let mut msg = MSG::default();
    //     while GetMessageW(&mut msg, None, 0, 0).into() {
    //         TranslateMessage(&msg);
    //         DispatchMessageW(&msg);
    //
    //         if msg.message == WM_QUIT {
    //             break;
    //         }
    //     }
    // }
}

// Função de processamento de mensagens da janela (WndProc)
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

// Função para converter string Rust (&str) para UTF-16
fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
