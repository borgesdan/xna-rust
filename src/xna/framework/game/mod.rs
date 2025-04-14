mod game_window;

use thiserror::Error;
use windows::core::imp::HMODULE;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use crate::xna::framework::Rectangle;

pub enum DisplayOrientation {
    Default,
    LandscapeLeft,
    LandscapeRight,
    Portrait,
}

#[derive(Error, Debug, Default)]
#[error("{message}")]
pub struct GameWindowError {
    pub message: String,
}

#[derive(Default, PartialEq, Eq)]
pub enum GameWindowStyle {
    #[default]
    Windowed,
    FullScreen,
    BorderlessFullScreen
}

#[derive(Default)]
pub struct GameWindow {
    pub window_pos_x: i32,
    pub window_pos_y: i32,
    pub window_height: i32,
    pub window_width: i32,
    pub window_title: String,
    pub window_style: GameWindowStyle,

    h_instance: isize,
    window_handle: isize,
    window_icon: isize,
    window_cursor: isize,
}