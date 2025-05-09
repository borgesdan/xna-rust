pub mod screen;

use crate::xna::csharp::Rectangle;

#[cfg(target_os = "windows")]
use crate::xna::platform::windows::WindowsScreen;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Screen {
    pub primary: bool,
    pub device_name: String,
    pub bounds: Rectangle,
    pub working_area: Rectangle,
    pub bit_depth: i32,

    #[cfg(target_os = "windows")]
    pub platform: WindowsScreen
}

pub struct SystemInformation;