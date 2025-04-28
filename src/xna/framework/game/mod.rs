pub mod game_window;

use std::io::IntoInnerError;
use std::ops::Deref;
use thiserror::Error;
use windows::core::imp::HMODULE;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use crate::xna::framework::{Color, Rectangle};
use crate::xna::framework::graphics::GraphicsDevice;

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
    pub inner_error: String
}

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum GameWindowStyle {
    #[default]
    Windowed,
    FullScreen,
    BorderlessFullScreen
}

#[derive(Default, PartialEq, Clone)]
pub struct GameWindow {
    pub window_pos_x: i32,
    pub window_pos_y: i32,
    pub window_height: i32,
    pub window_width: i32,
    pub window_title: String,
    pub window_style: GameWindowStyle,
}

#[derive(Default, PartialEq, Clone)]
pub struct Game {
    game_window: GameWindow,
    graphics_device: GraphicsDevice,
}