pub mod game;
pub mod game_window;
pub mod graphics_device_manager;

use std::cmp::Ordering;
use crate::xna::csharp::TimeSpan;
use crate::xna::framework::graphics::{DepthFormat, GraphicsAdapter, GraphicsDevice, PresentationParameters, SurfaceFormat};
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
    pub graphics_device: Option<Box<GraphicsDevice>>,
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

    pub is_device_dirty: bool,
    pub in_device_transition: bool,

    pub graphics_profile: GraphicsProfile,

    pub is_full_screen: bool,
    pub synchronize_with_vertical_retrace: bool,
    pub use_resized_back_buffer: bool,

    pub resized_back_buffer_width: u32,
    pub resized_back_buffer_height: u32,

    pub depth_stencil_format: DepthFormat,
    pub allow_multi_sampling: bool,

    pub back_buffer_format: SurfaceFormat,

    #[cfg(target_os = "windows")]
    pub platform: WindowsGraphicsDeviceManager
}

#[derive(Default, PartialEq, Clone, Copy, Eq)]
pub enum GraphicsProfile{
    #[default]
    Reach,
    HiDef
}

#[derive(Default, PartialEq, Clone, Eq)]
pub struct GraphicsDeviceInformation {
    pub adapter: GraphicsAdapter,
    pub profile: GraphicsProfile,
    pub presentation_parameters: PresentationParameters,
}

impl Ord for GraphicsDeviceInformation {
    fn cmp(&self, other: &Self) -> Ordering {
        //todo: implementar comparator

        if self != other{
            return if self.profile as i32 <= other.profile as i32 {Ordering::Less} else {Ordering::Greater}
        }

        let presentation_parameters1 = &self.presentation_parameters;
        let presentation_parameters2 = &other.presentation_parameters;

        if presentation_parameters1 != presentation_parameters2 && presentation_parameters1.multi_sample_count <= presentation_parameters2.multi_sample_count {
            return Ordering::Less;
        }

        Ordering::Equal
    }
}

impl PartialOrd for GraphicsDeviceInformation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        //todo: implementar comparator

        if self != other{
            return if self.profile as i32 <= other.profile as i32 {Some(Ordering::Less)} else {Some(Ordering::Greater)}
        }

        let presentation_parameters1 = &self.presentation_parameters;
        let presentation_parameters2 = &other.presentation_parameters;

        if presentation_parameters1 != presentation_parameters2 && presentation_parameters1.multi_sample_count <= presentation_parameters2.multi_sample_count {
            return Some(Ordering::Less);
        }

        Some (Ordering::Equal)
    }
}

impl SurfaceFormat {
    pub fn rank(format: &SurfaceFormat) -> i32 {
        match format {
            SurfaceFormat::Color => 32
        }
    }
}
