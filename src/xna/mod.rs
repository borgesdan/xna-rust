use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::xna::csharp::Exception;

pub mod framework;
pub mod csharp;
pub mod platform;

pub trait Unbox<T>: Sized {
    fn unbox(&self) -> Result<T, Exception>;
    fn unbox_ref(&self) -> Result<&T, Exception>;

    fn unbox_mut(&mut self) -> Result<&mut T, Exception>;
}

impl<T> Unbox<T> for Option<T> where T: Clone {
    fn unbox(&self) -> Result<T, Exception> {
        if self.is_none() {
            return Err(Exception::new("Invalid operation", None));
        }

        Ok(self.as_ref().unwrap().clone())
    }

    fn unbox_ref(&self) -> Result<&T, Exception> {
        if self.is_none() {
            return Err(Exception::new("Invalid operation", None));
        }

        Ok(self.as_ref().unwrap())
    }

    fn unbox_mut(&mut self) -> Result<&mut T, Exception> {
        if self.is_none() {
            return Err(Exception::new("Invalid operation", None));
        }

        Ok(self.as_mut().unwrap())
    }
}