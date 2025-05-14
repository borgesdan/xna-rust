mod stream;

use crate::xna::csharp::Exception;

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug)]
pub enum SeekOrigin {
    #[default]
    Begin,
    Current,
    End
}

trait Stream {
    fn get_can_read(&self) -> bool;
    fn get_can_write(&self) -> bool;
    fn get_can_seek(&self) -> bool;
    fn get_length(&self) -> i64;
    fn get_position(&self) -> i64;
    fn set_position(&self, value: i64) -> Result<(), Exception>;
    fn close(&mut self);
    fn flush(&mut self);
    fn seek(&mut self, offset: i64, origin: SeekOrigin) -> Result<(), Exception>;
    fn set_length(&mut self, value: i64) -> Result<(), Exception>;
    fn read(&self, buffer: &mut [u8], offset: i32, count: i32) -> Result<i32, Exception>;
    fn read_byte(&self) -> Result<i32, Exception>;
    fn write(&mut self, buffer: &[u8], offset: i32, count: i32) -> Result<i32, Exception>;
    fn write_byte(&mut self, value: u8)-> Result<i32, Exception>;
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
struct StreamHelper;