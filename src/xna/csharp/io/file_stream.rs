use crate::xna::csharp::io::{FileHelper, FileMode};
use crate::xna::csharp::io::FileStream;
use crate::xna::csharp::io::SeekOrigin;
use crate::xna::csharp::io::Stream;
use crate::xna::csharp::Exception;
use crate::xna::ExceptionConverter;
use std::fs;
use std::fs::File;
use std::io::{Seek, SeekFrom};

impl FileStream {
    pub fn new(filename: &str, mode: FileMode) -> Result<FileStream, Exception> {
        let file: Option<File> = match mode {
            FileMode::CreateNew => {
                Some(File::create_new(filename).unwrap_or_exception("Invalid create new operation.")?)
            }
            FileMode::Create => {
                Some(File::create(filename).unwrap_or_exception("Invalid create operation.")?)
            }
            FileMode::Open => {
                Some(File::open(filename).unwrap_or_exception("Invalid open operation.")?)
            }
            FileMode::OpenOrCreate => {
                if FileHelper::exists(filename)? {
                    Some(File::open(filename).unwrap_or_exception("Invalid open operation.")?)
                }else {
                    Some(File::create(filename).unwrap_or_exception("Invalid create operation.")?)
                }
            }
            FileMode::Truncate => {
                if FileHelper::exists(filename)? {
                    return Err(Exception::new("File not exists", None))
                }

                let f = File::create(filename).unwrap_or_exception("Invalid create operation.")?;
                f.set_len(0).unwrap_or_exception("File truncation error.")?;

                Some(f)
            }
            FileMode::Append => {
                if FileHelper::exists(filename)? {
                    let mut f = File::open(filename)
                        .unwrap_or_exception("Invalid open operation.")?;

                    f.seek(SeekFrom::End(0))
                        .unwrap_or_exception("Invalid seek operation")?;

                    Some(f);
                }

                Some(File::create(filename).unwrap_or_exception("Invalid create operation to append.")?)
            }
        };

        Ok(FileStream {
            file
        })
    }

    pub fn delete_file(filename: &str) -> Result<(), Exception> {
        fs::remove_file(filename).unwrap_or_exception("File not deleted.")
    }
}

impl Stream for FileStream {
    fn get_can_read(&self) -> Result<bool, Exception> {
        todo!()
    }

    fn get_can_write(&self) -> Result<bool, Exception> {
        todo!()
    }

    fn get_can_seek(&self) -> Result<bool, Exception> {
        todo!()
    }

    fn get_length(&self) -> Result<i64, Exception> {
        todo!()
    }

    fn get_position(&self) -> Result<i64, Exception> {
        todo!()
    }

    fn set_position(&mut self, value: i64) -> Result<(), Exception> {
        todo!()
    }

    fn close(&mut self) -> Result<(), Exception> {
        todo!()
    }

    fn flush(&mut self) -> Result<(), Exception> {
        todo!()
    }

    fn seek(&mut self, offset: i64, origin: SeekOrigin) -> Result<i64, Exception> {
        todo!()
    }

    fn set_length(&mut self, value: i64) -> Result<(), Exception> {
        todo!()
    }

    fn read(&mut self, buffer: &mut [u8], offset: i32, count: i32) -> Result<i32, Exception> {
        todo!()
    }

    fn read_byte(&mut self) -> Result<i32, Exception> {
        todo!()
    }

    fn write(&mut self, buffer: &[u8], offset: i32, count: i32) -> Result<(), Exception> {
        todo!()
    }

    fn write_byte(&mut self, value: u8) -> Result<(), Exception> {
        todo!()
    }

    fn copy_to(&mut self, destination: &mut dyn Stream, buffer_size: i32) -> Result<(), Exception> {
        todo!()
    }

    fn write_to(&mut self, stream: &mut dyn Stream) -> Result<(), Exception> {
        todo!()
    }
}