use std::error::Error;
use crate::xna::csharp::{Exception, HResult};

impl Exception {
    pub fn new(message: &str, inner: Option<Exception>) -> Self {
        Self::new_with_hresult(message, HResult::COR_E_EXCEPTION.0, inner)
    }

    pub fn new_with_hresult(message: &str, h_result: i64, inner: Option<Exception>) -> Self {
        Exception {
            message: message.to_string(),
            inner: if inner.is_some() { Some(Box::new(inner.unwrap())) } else { None },
            h_result,
        }
    }
}