pub mod game;
pub mod game_window;

use std::io::IntoInnerError;
use std::ops::Deref;
use thiserror::Error;
use windows::core::imp::HMODULE;
use windows::Win32::UI::WindowsAndMessaging::HICON;
use crate::xna::csharp::TimeSpan;
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

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub struct GameTime {
    pub elapsed_time: TimeSpan,
    pub is_slowly: bool,
    pub total_time: TimeSpan,
}

#[derive(Default, PartialEq, Clone)]
pub struct Game {
    pub game_window: Option<GameWindow>,
    pub graphics_device: Option<GraphicsDevice>,
    pub target_elapsed_time: TimeSpan,
    pub current_game_time: GameTime,
    pub is_fixed_time_step: bool,
}