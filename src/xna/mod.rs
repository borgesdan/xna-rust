use crate::xna::csharp::Exception;
use std::ops::Deref;

pub mod framework;
pub mod csharp;
pub mod platform;

pub trait ExceptionConverter<T> {
    fn unwrap_or_exception(self, message: &str) -> Result<T, Exception>;
    fn unwrap_ref_or_exception(&self, message: &str) -> Result<&T, Exception>;
    fn unwrap_mut_or_exception(&mut self, message: &str) -> Result<&mut T, Exception>;
}

impl<T> ExceptionConverter<T> for Option<T> {
    fn unwrap_or_exception(self, message: &str) -> Result<T, Exception> {
        if self.is_some() {
            return Ok(self.unwrap())
        }

        Err(Exception::new(message, None))
    }

    fn unwrap_ref_or_exception(&self, message: &str) -> Result<&T, Exception> {
        if self.is_some() {
            return Ok(self.as_ref().unwrap());
        }

        Err(Exception::new(message, None))
    }

    fn unwrap_mut_or_exception(&mut self, message: &str) -> Result<&mut T, Exception> {
        if self.is_some() {
            return Ok(self.as_mut().unwrap())
        }

        Err(Exception::new(message, None))
    }
}

pub trait SilentExceptionConverter<T> {
    fn unwrap_or_default_exception(self) -> Result<T, Exception>;
    fn unwrap_ref_or_default_exception(&self) -> Result<&T, Exception>;
    fn unwrap_mut_or_default_exception(&mut self) -> Result<&mut T, Exception>;
}

impl<T> SilentExceptionConverter<T> for Option<T> {
    fn unwrap_or_default_exception(self) -> Result<T, Exception> {
        if self.is_some() {
            return Ok(self.unwrap())
        }

        Err(Exception::new("Invalid unwrap() operation.", None))
    }

    fn unwrap_ref_or_default_exception(&self) -> Result<&T, Exception> {
        if self.is_some() {
            return Ok(self.as_ref().unwrap());
        }

        Err(Exception::new("Invalid .as_ref().unwrap() operation.", None))
    }

    fn unwrap_mut_or_default_exception(&mut self) -> Result<&mut T, Exception> {
        if self.is_some() {
            return Ok(self.as_mut().unwrap())
        }

        Err(Exception::new("Invalid .as_mut().unwrap() operation", None))
    }
}