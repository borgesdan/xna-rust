pub mod forms;

use std::str;
use thiserror::Error;

#[derive(Default)]
struct Rectangle {
    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32,
}

impl Rectangle {
    pub fn from_ltrb(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Rectangle{
            x: left,
            y: top,
            width: right - left,
            height: bottom - top
        }
    }
}