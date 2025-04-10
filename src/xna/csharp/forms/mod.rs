pub mod screen;

use thiserror::Error;
use crate::xna::csharp::Rectangle;
pub struct Screen {
    pub h_monitor: isize,
    pub primary: bool,
    pub device_name: String,
    pub bounds: Rectangle,
    pub working_area: Rectangle
}

#[derive(Error, Debug, Default)]
#[error("{message}")]
pub struct ScreenError{
    pub message: String,
}