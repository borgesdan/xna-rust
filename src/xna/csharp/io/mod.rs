pub mod stream;
pub mod memory_stream;

use crate::xna::csharp::Exception;

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug)]
pub enum SeekOrigin {
    #[default]
    Begin,
    Current,
    End
}

pub trait Stream {
    fn get_can_read(&self) -> Result<bool, Exception>;
    fn get_can_write(&self) -> Result<bool, Exception>;
    fn get_can_seek(&self) -> Result<bool, Exception>;
    fn get_length(&self) -> Result<i64, Exception>;
    fn get_position(&self) -> Result<i64, Exception>;
    fn set_position(&mut self, value: i64) -> Result<(), Exception>;
    fn close(&mut self)-> Result<(), Exception>;
    fn flush(&mut self)-> Result<(), Exception>;
    fn seek(&mut self, offset: i64, origin: SeekOrigin) -> Result<i64, Exception>;
    fn set_length(&mut self, value: i64) -> Result<(), Exception>;
    fn read(&mut self, buffer: &mut [u8], offset: i32, count: i32) -> Result<i32, Exception>;
    fn read_byte(&mut self) -> Result<i32, Exception>;
    fn write(&mut self, buffer: &[u8], offset: i32, count: i32) -> Result<(), Exception>;
    fn write_byte(&mut self, value: u8)-> Result<(), Exception>;
    fn copy_to(&mut self, destination: &mut dyn Stream, buffer_size: i32) -> Result<(), Exception>;
    fn write_to(&mut self, stream: &mut dyn Stream) -> Result<(), Exception>;
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct StreamHelper;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct MemoryStream {
    buffer: Vec<u8>,
    origin: i32,
    position: i32,
    length: i32,
    capacity: i32,
    expandable: bool,
    writable: bool,
    exposable: bool,
    is_open: bool
}