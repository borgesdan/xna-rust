pub mod forms;
pub mod time_span;

use std::str;
use thiserror::Error;

#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub struct Rectangle {
    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32,
}

#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub struct TimeSpan {
    ticks: i64,
}

#[derive(Error, Debug, Default, Eq, PartialEq, Clone)]
#[error("{h_result}: {message}")]
pub struct Exception {
    pub message: String,
    pub inner: Box<Exception>,
    pub h_result: isize,
}

impl Exception {
    pub fn new_out_of_range(message: &str, inner: Option<Exception>) -> Self {
        Exception {
            message: message.to_string(),
            inner: Box::default(),
            h_result: 0x80004003,
        }
    }
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
