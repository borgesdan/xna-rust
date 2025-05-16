pub mod forms;
pub mod time_span;
pub mod io;
pub mod buffer;
pub mod array;
pub mod exception;
pub mod rectangle;
pub mod hresult;

use std::error;
use std::error::Error;
use thiserror::Error;

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct Rectangle {
    pub x : i32,
    pub y : i32,
    pub width : i32,
    pub height : i32,
}

#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub struct TimeSpan {
    pub ticks: i64,
}

#[derive(Error, Debug, Default)]
#[error("{h_result}: {message}")]
pub struct Exception {
    pub message: String,
    pub inner: Option<Box<Exception>>,
    pub h_result: i64,
}

pub struct Buffer;
pub struct Array;

pub struct HResult(pub i64);


