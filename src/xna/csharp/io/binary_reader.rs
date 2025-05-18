use crate::xna::csharp::Exception;
use crate::xna::csharp::io::{BinaryReader, IBinaryReader, Stream};
use crate::xna::{pointer_to_numeric, ExceptionConverter};

impl<T> BinaryReader<T> where T : Stream {
    pub fn new(input: T, leave_open: bool) -> Self {
        BinaryReader {
            stream: Some(input),
            leave_open,
            internal_buffer: [0;16]
        }
    }

    fn throw_if_closed(&self) -> Result<(), Exception> {
        if self.stream.is_none(){
            return Err(Exception::new("Stream is closed.", None));
        }

        Ok(())
    }

    pub fn get_stream(&self) -> Result<&T, Exception> {
        self.throw_if_closed()?;
        self.stream.unwrap_ref_or_exception("Stream is closed")
    }

    pub fn get_stream_mut(&mut self) -> Result<&mut T, Exception> {
        self.throw_if_closed()?;
        self.stream.unwrap_mut_or_exception("Stream is closed")
    }

    fn internal_read_byte(&mut self) -> Result<u8, Exception> {
        self.throw_if_closed()?;
        let mut stream = self.get_stream_mut()?;

        let b = stream.read_byte()?;

        if b == -1 {
            return Err(Exception::new("Error, end of file.", None));
        }

        Ok(b as u8)
    }

    fn read_numeric<TNumber>(&mut self) -> Result<TNumber, Exception>{
        let size = size_of::<T>();
        let mut stream = self.get_stream_mut()?;
        let mut buffer = vec![0; size];

        stream.read(buffer.as_mut_slice(), 0, size as i32)?;

        let ptr = buffer.as_ptr();

        let value = pointer_to_numeric::<TNumber>(ptr)?;

        Ok(value)
    }
}

impl<T> IBinaryReader for BinaryReader<T> where T : Stream {
    fn close(&mut self) -> Result<(), Exception> {
        if !self.leave_open && self.stream.is_some() {
            self.stream.as_mut().unwrap().close()?;
        }

        self.stream = None;

        Ok(())
    }

    fn peek_char(&mut self) -> Result<i32, Exception> {
        self.throw_if_closed()?;

        if !self.stream.as_ref().unwrap().get_can_seek()? {
            return Ok(-1);
        }

        let orig_pos = self.stream.
            as_mut().unwrap()
            .get_position()?;

        let ch = self.read()?;

        self.stream
            .as_mut().unwrap()
            .set_position(orig_pos)?;

        Ok(ch)
    }

    fn read(&mut self) -> Result<i32, Exception> {
        self.throw_if_closed()?;
        let stream = self.get_stream_mut()?;

        let byte = stream.read_byte()?;

        Ok(byte)
    }

    fn read_byte(&mut self) -> Result<u8, Exception> {
        self.internal_read_byte()
    }

    fn read_sbyte(&mut self) -> Result<i8, Exception> {
        let byte = self.internal_read_byte()?;
        Ok(byte as i8)
    }

    fn read_boolean(&mut self) -> Result<bool, Exception> {
        Ok(self.internal_read_byte()? != 0)
    }

    fn read_char(&mut self) -> Result<char, Exception> {
        let value = self.read()?;

        if value == -1 {
            return Err(Exception::new("Error, end of file.", None));
        }

        let c = (value as u8) as char;
        Ok(c)
    }

    fn read_int16(&mut self) -> Result<i16, Exception> {
        self.throw_if_closed()?;
        self.read_numeric::<i16>()
    }
}