pub mod stream;
pub mod memory_stream;
mod file_stream;
mod file;

use std::fs::File;
use windows::core::Error;
use crate::xna::csharp::Exception;
use crate::xna::ExceptionConverter;

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

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum FileMode
{
    /// Creates a new file. An exception is raised if the file already exists.
    #[default]
    CreateNew = 1,
    /// Creates a new file. If the file already exists, it is overwritten.
    Create = 2,
    /// Opens an existing file. An exception is raised if the file does not exist.
    Open = 3,
    /// Opens the file if it exists. Otherwise, creates a new file.
    OpenOrCreate = 4,
    /// Opens an existing file. Once opened, the file is truncated so that its
    /// size is zero bytes. The calling process must open the file with at least
    /// WRITE access. An exception is raised if the file does not exist.
    Truncate = 5,
    /// Opens the file if it exists and seeks to the end.  Otherwise,
    /// creates a new file.
    Append = 6,
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub enum FileAccess
{
    /// Specifies read access to the file. Data can be read from the file and
    /// the file pointer can be moved. Combine with WRITE for read-write access.
    #[default]
    Read = 1,

    /// Specifies write access to the file. Data can be written to the file and
    /// the file pointer can be moved. Combine with READ for read-write access.
    Write = 2,

    /// Specifies read and write access to the file. Data can be written to the
    /// file and the file pointer can be moved. Data can also be read from the
    /// file.
    ReadWrite = 3,
}

#[derive(Debug, Default)]
pub struct FileStream {
    file: Option<File>,
    access: FileAccess,
    is_open: bool,
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy)]
pub struct FileHelper;

impl<T> ExceptionConverter<T> for Result<T, std::io::Error>{
    fn unwrap_or_exception(self, message: &str) -> Result<T, Exception> {
        if self.is_ok() {
            return Ok(self.unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let inner = Exception::from(error.clone());
        let exception = Exception::new_with_hresult(message, 0, Some(inner));

        Err(exception)
    }

    fn unwrap_ref_or_exception(&self, message: &str) -> Result<&T, Exception> {
        if self.is_ok() {
            return Ok(self.as_ref().unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let inner = Exception::from(error.clone());
        let exception = Exception::new_with_hresult(message, 0, Some(inner));

        Err(exception)
    }

    fn unwrap_mut_or_exception(&mut self, message: &str) -> Result<&mut T, Exception> {
        if self.is_ok() {
            return Ok(self.as_mut().unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let inner = Exception::from(error.clone());
        let exception = Exception::new_with_hresult(message, 0, Some(inner));

        Err(exception)
    }
}

impl From<std::io::Error> for Exception {
    fn from(value: std::io::Error) -> Self {
        let message = value.to_string();
        let code = 0;

        Exception::new_with_hresult(message.as_str(), code, None)
    }
}

impl From<&std::io::Error> for Exception {
    fn from(value: &std::io::Error) -> Self {
        let message = value.to_string();
        let code = 0;

        Exception::new_with_hresult(message.as_str(), code, None)
    }
}
