use crate::xna::csharp::Exception;
use crate::xna::csharp::io::{FileHelper, FileMode, FileStream};
use crate::xna::ExceptionConverter;

impl FileHelper {
    pub fn exists(path: &str) -> Result<bool, Exception> {
        let file = std::fs::metadata(path).unwrap_or_exception("Cannot access file.");
        Ok(file.is_ok())
    }

    pub fn create(path: &str) -> Result<FileStream, Exception> {
        FileStream::new(path, FileMode::Create)
    }

    pub fn delete(path: &str) -> Result<(), Exception> {
        FileStream::delete_file(path)
    }

    pub fn open(path: &str, mode: FileMode) -> Result<FileStream, Exception> {
        FileStream::new(path, mode)
    }
}