use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::xna::csharp::Exception;

pub mod framework;
pub mod csharp;
pub mod platform;

pub trait UnboxRc<T>: Sized {
    fn unbox(&self) -> Result<Rc<RefCell<T>>, Exception>;
}

impl<T> UnboxRc<T> for Option<Rc<RefCell<T>>> {
    fn unbox(&self) -> Result<Rc<RefCell<T>>, Exception> {
        if self.is_none() {
            return Err(Exception::new("Invalid operation", None));
        }


        Ok(self.as_ref().unwrap().clone())
    }
}