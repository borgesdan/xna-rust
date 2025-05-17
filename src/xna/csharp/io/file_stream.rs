use crate::xna::csharp::io::{FileAccess, FileHelper, FileMode, StreamHelper};
use crate::xna::csharp::io::FileStream;
use crate::xna::csharp::io::SeekOrigin;
use crate::xna::csharp::io::Stream;
use crate::xna::csharp::Exception;
use crate::xna::ExceptionConverter;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

impl FileStream {
    pub fn new(filename: &str, mode: FileMode) -> Result<FileStream, Exception> {
        Self::new_with_access(filename, mode, FileAccess::ReadWrite)
    }
    pub fn new_with_access(filename: &str, mode: FileMode, access: FileAccess) -> Result<FileStream, Exception> {
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
            file,
            access,
            is_open: true,
        })
    }

    pub fn delete_file(filename: &str) -> Result<(), Exception> {
        fs::remove_file(filename).unwrap_or_exception("File not deleted.")
    }

    fn write_to(&mut self, stream: &mut dyn Stream) -> Result<(), Exception> {
        let pos = self.get_position()?;
        let length = self.get_length()?;
        let buffer = vec![0u8; length as usize];
        self.set_position(0)?;

        stream.write(&buffer, 0, length as i32)?;

        self.set_position(pos)?;

        Ok(())
    }
}

impl Stream for FileStream {
    fn get_can_read(&self) -> Result<bool, Exception> {
        let result = self.is_open && self.file.is_some() && (self.access == FileAccess::Read || self.access == FileAccess::ReadWrite);

        Ok(result)
    }

    fn get_can_write(&self) -> Result<bool, Exception> {
        let result = self.is_open && self.file.is_some() && (self.access == FileAccess::Write || self.access == FileAccess::ReadWrite);

        Ok(result)
    }

    fn get_can_seek(&self) -> Result<bool, Exception> {
        _ = self.get_can_read()?;

        let result = self.is_open && self.file.is_some();
        Ok(result)
    }

    fn get_length(&self) -> Result<i64, Exception> {
        _ = self.get_can_read()?;

        let len = self.file
            .unwrap_ref_or_exception("Cannot get length of file")?
            .metadata()
            .unwrap_or_exception("Cannot get length of file.")?
            .len();

        Ok(len as i64)
    }

    fn get_position(&self) -> Result<i64, Exception> {
        _ = self.get_can_read()?;

        let pos =  self.file
            .unwrap_ref_or_exception("Cannot get length of file")?
            .stream_position()
            .unwrap_or_exception("Cannot get position of file.")?;

        Ok(pos as i64)
    }

    fn set_position(&mut self, value: i64) -> Result<(), Exception> {
        _ = self.get_can_write()?;

        self.file
            .unwrap_ref_or_exception("Cannot set position of file.")?
            .seek(SeekFrom::Start(value as u64))
            .unwrap_or_exception("Cannot set position of file.")?;

        Ok(())
    }

    fn close(&mut self) -> Result<(), Exception> {
        self.is_open = false;
        self.file = None;

        Ok(())
    }

    fn flush(&mut self) -> Result<(), Exception> {
        _ = self.get_can_write()?;

        self.file
            .unwrap_ref_or_exception("Cannot flush file.")?
            .flush()
            .unwrap_or_exception("Cannot flush file.")?;

        Ok(())
    }

    fn seek(&mut self, offset: i64, origin: SeekOrigin) -> Result<i64, Exception> {
        _ = self.get_can_seek()?;

        let seek_from = match origin {
            SeekOrigin::Begin => {
                SeekFrom::Start(offset as u64)
            }
            SeekOrigin::Current => {
                SeekFrom::Current(offset)
            }
            SeekOrigin::End => {
                SeekFrom::End(offset)
            }
        };

        let mut file = self.file
            .unwrap_mut_or_exception("Cannot seek file.")?;

        file.seek(seek_from)
            .unwrap_or_exception("Cannot seek file.")?;

        let pos =  file.stream_position().unwrap_or_exception("Cannot seek file.")?;

        Ok(pos as i64)
    }

    fn set_length(&mut self, value: i64) -> Result<(), Exception> {
        _ = self.get_can_write()?;

        self.file
            .unwrap_mut_or_exception("Cannot set length.")?
            .set_len(value as u64)
            .unwrap_or_exception("Cannot set length of file.")?;

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8], offset: i32, count: i32) -> Result<i32, Exception> {
        _ = self.get_can_read()?;

        let mut file = self.file.unwrap_mut_or_exception("Cannot read file.")?;
        let slice = &mut buffer[offset as usize..count as usize];
        let result = file.read(slice).unwrap_or_exception("Cannot read file.")?;

        Ok(result as i32)
    }

    fn read_byte(&mut self) -> Result<i32, Exception> {
        _ = self.get_can_read()?;

        let mut buffer = [0u8; 1];
        self.read(&mut buffer, 0, 1)
    }

    fn write(&mut self, buffer: &[u8], offset: i32, count: i32) -> Result<(), Exception> {
        _ = self.get_can_write()?;

        let slice = &buffer[offset as usize..count as usize];
        self.file
            .unwrap_ref_or_exception("Cannot write file.")?
            .write(slice).unwrap_or_exception("Cannot write file.")?;

        Ok(())
    }

    fn write_byte(&mut self, value: u8) -> Result<(), Exception> {
        _ = self.get_can_write()?;

        let buffer = [value; 1];
        self.write(&buffer, 0, 1)
    }

    fn copy_to(&mut self, destination: &mut dyn Stream, buffer_size: i32) -> Result<(), Exception> {
        StreamHelper::validate_copy_to_arguments(destination, buffer_size)?;

        let position = self.get_position()?;

        let mut buffer = vec![0u8; buffer_size as usize];

        self.file
            .unwrap_ref_or_exception("Cannot copy file.")?
            .read_exact(&mut buffer)
            .unwrap_or_exception("Cannot copy file.")?;

        self.set_position(position)?;
        destination.write(buffer.as_slice(), position as i32, buffer_size)?;

        Ok(())
    }
}