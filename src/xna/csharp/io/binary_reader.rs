use crate::xna::csharp::Exception;
use crate::xna::csharp::io::{BinaryReader, IBinaryReader, Stream};

impl<T> BinaryReader<T> where T : Stream {
    pub fn new(input: T, leave_open: bool) -> Self {
        BinaryReader {
            stream: Some(input),
            leave_open,
        }
    }

    fn throw_if_closed(&self) -> Result<(), Exception> {
        if self.stream.is_none(){
            return Err(Exception::new("Stream is closed.", None));
        }

        Ok(())
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
        todo!()
    }
}