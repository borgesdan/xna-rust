mod xna;

use std::cell::RefCell;
use std::fmt::Pointer;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LRESULT, WPARAM, LPARAM, HINSTANCE};
use windows::Win32::System::LibraryLoader::{GetModuleHandleW};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage,
    RegisterClassExW, ShowWindow, TranslateMessage, MSG, WNDCLASSEXW,
    CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, SW_SHOW, WM_DESTROY, WM_PAINT,
    WM_QUIT, WS_OVERLAPPEDWINDOW, WINDOW_EX_STYLE};

use crate::xna::framework::{Color, Point};
use crate::xna::csharp::forms::Screen;
use crate::xna::framework::game::{Game, GameWindow, GameWindowStyle, GraphicsDeviceManager};
use crate::xna::framework::graphics::{GraphicsAdapter, GraphicsAdapterOutput, GraphicsDevice, PresentationParameters, SurfaceFormat, SwapEffect};

fn main() {

    let mut game = Rc::new(RefCell::new(Game::new()));

    let mut graphics_device_manager = Box::new(GraphicsDeviceManager::new(Some(game.clone())));

    //let apply = graphics_device_manager.apply_changes();
    game.borrow_mut().run(&mut graphics_device_manager);


    // let adapter = GraphicsAdapter::default_adapter();
    //
    // let g_device = GraphicsDevice::new();
    // let mut device = g_device.create();
    // let window = GameWindow::create_window(Point{ x: 800, y: 600}, GameWindowStyle::Windowed, "Teste" ).unwrap();
    //
    // device.base.presentation_parameters = PresentationParameters {
    //     back_buffer_format: SurfaceFormat::Color,
    //     back_buffer_height: 600,
    //     back_buffer_width: 800,
    //     is_full_screen: false,
    //     presentation_swap_effect: SwapEffect::FlipDiscard,
    //
    //     ..Default::default()
    // };
    //
    // device.parameters.hwnd = window.hwnd;
    //
    // device.initialize();



    // let mut msg = MSG::default();
    // unsafe {
    //     while GetMessageW(&mut msg, None, 0, 0).into() {
    //         TranslateMessage(&msg);
    //         DispatchMessageW(&msg);
    //
    //         if msg.message == WM_QUIT {
    //             break;
    //         }
    //     }
    // }
return;



    unsafe {
        // Nome da classe da janela
        let class_name = to_wide("MinhaJanelaClass");

        // Obter instância do programa
        let h_module = GetModuleHandleW(None).unwrap();
        let h_instance = HINSTANCE::from(h_module);

        // Definir classe da janela
        let wnd_class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc1), // Ponteiro para o callback de eventos da janela
            hInstance: h_instance.into(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            //hbrBackground: HBRUSH(16), // Cor de fundo padrão
            ..Default::default()
        };

        // Registrar a classe da janela
        let result = RegisterClassExW(&wnd_class);

        // Criar a janela
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(to_wide("Minha Primeira Janela!").as_ptr()),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800, // Largura
            600, // Altura
            None,
            None,
            Some(h_instance),
            None,
        ).unwrap();

        // Exibir a janela
        ShowWindow(hwnd, SW_SHOW);

        // Loop de mensagens
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);

            if msg.message == WM_QUIT {
                break;
            }
        }
    }
}

//Função de processamento de mensagens da janela (WndProc)
extern "system" fn wnd_proc1(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
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

//unção para converter string Rust (&str) para UTF-16
fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
