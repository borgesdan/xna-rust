pub mod screen;

use crate::xna::csharp::Rectangle;
use crate::xna::platform::PlatformScreen;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct Screen {
    pub primary: bool,
    pub device_name: String,
    pub bounds: Rectangle,
    pub working_area: Rectangle,
    pub bit_depth: i32,

    pub platform: PlatformScreen
}

pub struct SystemInformation;