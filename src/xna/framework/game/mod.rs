pub mod game;
pub mod game_window;
pub mod graphics_device_manager;

use crate::xna::csharp::TimeSpan;
use crate::xna::framework::graphics::{GraphicsAdapter, GraphicsDevice, PresentationParameters};
use std::ops::Deref;
use thiserror::Error;

#[cfg(target_os = "windows")]
use crate::xna::platform::windows::{WindowsGame, WindowsGameWindow, WindowsGraphicsDeviceManager};

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
    pub x: i32,
    pub y: i32,
    pub height: i32,
    pub width: i32,
    pub title: String,
    pub style: GameWindowStyle,

    #[cfg(target_os = "windows")]
    pub platform: WindowsGameWindow
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

    #[cfg(target_os = "windows")]
    pub platform: WindowsGame
}

#[derive(Default, PartialEq, Clone)]
pub struct GraphicsDeviceManager {
    pub presentation_parameters: PresentationParameters,

    pub game: Option<Box<Game>>,
    pub graphics_adapter: Option<Box<GraphicsAdapter>>,
    pub graphics_device: Option<Box<GraphicsDevice>>,

    pub device_dirty: bool,
    pub in_device_transition: bool,

    #[cfg(target_os = "windows")]
    pub platform: WindowsGraphicsDeviceManager
}
