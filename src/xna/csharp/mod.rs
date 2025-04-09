pub mod forms;

use std::str;

#[derive(Default)]
struct Rectangle {
    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32,
}

pub struct Screen {
    h_monitor: isize,
    primary: bool,
    device_name: String,
    bounds: Rectangle,
    working_area: Rectangle
}

impl Rectangle {
    pub fn from_ltrb(left: i32, top: i32, right: i32, bottom: i32) -> Rectangle {
        Rectangle{
            x: left,
            y: top,
            width: right - left,
            height: bottom - top
        }
    }
}