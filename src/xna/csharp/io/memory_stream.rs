use crate::xna::csharp::io::{MemoryStream, SeekOrigin, Stream, StreamHelper};
use crate::xna::csharp::{Array, Buffer, Exception};
use std::cmp::max;

impl MemoryStream {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: i32) -> Self {
        let capacity = if capacity < 0 { 0 } else { capacity };

        MemoryStream {
            buffer: if capacity != 0 { Vec::with_capacity(capacity as usize)} else { Vec::new() },
            capacity,
            expandable: true,
            writable: true,
            exposable: true,
            is_open: true,
            ..Default::default()
        }
    }

    pub fn with_buffer(buffer: &[u8], writable: bool) -> Self {
        MemoryStream {
            buffer: buffer.to_vec(),
            writable,
            length: buffer.len() as i32,
            capacity: buffer.len() as i32,
            is_open: true,
            ..Default::default()
        }
    }

    pub fn with_all(buffer: &[u8], index: i32, count: i32, writable: bool, publicly_visible: bool) -> Result<Self, Exception> {
        let index = if index < 0 { 0 } else { index };
        let count = if count < 0 { 0 } else { count };

        if buffer.len() as i32 - index < count {
            return Err(Exception::new("Invalid off len.", None));
        }

       Ok(MemoryStream {
            buffer: buffer.to_vec(),
            origin: index,
            position: index,
            length: index + count,
            capacity: index + count,
            writable,
            exposable: publicly_visible,
            is_open: true,
            ..Default::default()
        })
    }

    fn ensure_not_close(&self) -> Result<(), Exception> {
        if !self.is_open {
            return Err(Exception::new("Stream is closed.", None));
        }

        Ok(())
    }

    fn ensure_writable(&self) -> Result<(), Exception> {
        if !self.get_can_write()? {
            return Err(Exception::new("Unwritable stream.", None));
        }

        Ok(())
    }

    fn ensure_capacity(&mut self, value: i32) -> Result<bool, Exception> {
        if value < 0 {
            return Err(Exception::new("Invalid capacity.", None));
        }

        if value > self.capacity {
            let mut new_capacity = max(value, 256);

            if new_capacity < self.capacity * 2 {
                new_capacity = self.capacity * 2;
            }

            if (self.capacity * 2) as u32 > 0x7FFFFFC7 {
                new_capacity = max(value,0x7FFFFFC7);
            }

            self.set_capacity(new_capacity)?;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn get_capacity(&self) -> Result<i32, Exception> {
        self.ensure_not_close()?;
        Ok(self.capacity - self.origin)
    }

    pub fn set_capacity(&mut self, value: i32) -> Result<(), Exception> {
        if value < self.get_length()? as i32 {
            return Err(Exception::new("Invalid capacity.", None));
        }

        self.ensure_not_close()?;

        if !self.expandable && (value != self.get_capacity()?) {
            return Err(Exception::new("Invalid capacity.", None));
        }

        if self.exposable && value != self.capacity {
            if value > 0 {
                let mut new_buffer:Vec<u8> = Vec::with_capacity(value as usize);
                let slice = new_buffer.as_mut_slice();

                if self.length > 0 {
                    Buffer::block_copy(&self.buffer, 0, slice, 0, self.length as usize)?;
                }

                self.buffer.copy_from_slice(&new_buffer);

            } else {
                self.buffer = Vec::new();
            }

            self.capacity = value;
        }

        Ok(())
    }

    pub fn get_buffer(&self) -> Result<Vec<u8>, Exception> {
        if self.exposable {
            return Err(Exception::new("Unauthorized access.", None));
        }

        Ok(self.buffer.clone())
    }

    fn internal_emulated_read(&mut self, count: i32) -> Result<i32, Exception> {
        self.ensure_not_close()?;

        let mut n = self.length - self.position;

        if n > count {
            n = count;
        }
        if n < 0 {
            n = 0
        }

        self.position = self.position + n;

        Ok(n)
    }

    pub const MEM_STREAM_MAX_LENGTH : i32 = i32::MAX;
}

impl Stream for MemoryStream {
    fn get_can_read(&self) -> Result<bool, Exception> {
        Ok(self.is_open)
    }

    fn get_can_write(&self) -> Result<bool, Exception> {
        Ok(self.writable)
    }

    fn get_can_seek(&self) -> Result<bool, Exception> {
        Ok(self.is_open)
    }

    fn get_length(&self) -> Result<i64, Exception> {
        self.ensure_not_close()?;

        Ok((self.length - self.origin) as i64)
    }

    fn get_position(&self) -> Result<i64, Exception> {
        self.ensure_not_close()?;
        Ok((self.position - self.origin) as i64)
    }

    fn set_position(&mut self, value: i64) -> Result<(), Exception> {
        let value = if value < 0 { 0 } else { value };

        self.ensure_not_close()?;

        if value > (Self::MEM_STREAM_MAX_LENGTH - self.origin) as i64 {
            return Err(Exception::new("Invalid position.", None));
        }

        self.position = self.origin + value as i32;

        Ok(())
    }

    fn close(&mut self)-> Result<(), Exception> {
        self.is_open = false;
        self.writable = false;
        self.expandable = false;

        Ok(())
    }

    fn flush(&mut self)-> Result<(), Exception> {
        Ok(())
    }

    fn seek(&mut self, offset: i64, origin: SeekOrigin) -> Result<i64, Exception> {
        self.ensure_not_close()?;

        let loc = match origin {
            SeekOrigin::Begin => self.origin,
            SeekOrigin::Current => self.position,
            SeekOrigin::End => self.length
        };

        if offset > (Self::MEM_STREAM_MAX_LENGTH - loc) as i64 {
            return Err(Exception::new("Offset: Out of range.", None));
        }

        let temp_position = loc + offset as i32;

        if loc as i64 + offset < self.origin as i64 || temp_position < self.origin {
            return Err(Exception::new("Invalid seek: seek before begin.", None));
        }

        self.position = temp_position;

        Ok((self.position < self.origin) as i64)
    }

    fn set_length(&mut self, value: i64) -> Result<(), Exception> {
        if value < 0 || value > i32::MAX as i64 {
            return Err(Exception::new("Value: Out of range.", None));
        }

        self.ensure_writable()?;

        if value > (i32::MAX - self.origin) as i64 {
            return Err(Exception::new("Value: Out of range.", None));
        }

        let new_length = self.origin + value as i32;
        let allocated_new_array = self.ensure_capacity(new_length)?;

        if !allocated_new_array && new_length > self.length {
           Array::clear(&mut self.buffer, self.length as usize, (new_length - self.length) as usize)?;
        }

        self.length = new_length;

        if self.position > new_length {
            self.position = new_length
        }

        Ok(())
    }

    fn read(&mut self, buffer: &mut [u8], offset: i32, count: i32) -> Result<i32, Exception> {
        StreamHelper::validate_buffer_arguments(&buffer, offset, count)?;
        self.ensure_not_close()?;

        let mut n = self.length - self.position;

        if n > count {
            n = count;
        }

        if n <= 0 {
            return Ok(0);
        }

        if n <= 8 {
            let mut byte_count = n as usize;

            loop {
                byte_count = byte_count - 1;

                if byte_count >= 0 {
                    buffer[offset as usize + byte_count] = self.buffer[self.position as usize + byte_count];
                }else {
                    break;
                }
            }
        }else {
            Buffer::block_copy(&self.buffer, self.position as usize, buffer, offset as usize, n as usize)?;
        }
        self.position = self.position + n as i32;

        Ok(n)
    }

    fn read_byte(&mut self) -> Result<i32, Exception> {
       self.ensure_not_close()?;

        if self.position >= self.length {
            return Ok(-1);
        }

        let result = self.buffer[self.position as usize];
        self.position = self.position + 1;

        Ok(result as i32)
    }

    fn write(&mut self, buffer: &[u8], offset: i32, count: i32) -> Result<(), Exception> {
        StreamHelper::validate_buffer_arguments(buffer, offset, count)?;
        self.ensure_not_close()?;
        self.ensure_writable()?;

        let i = self.position + count;

        if i < 0 {
            return Err(Exception::new("Invalid write.", None));
        }

        if i > self.length {
            let mut must_zero = self.position > self.length;

            if i > self.capacity {
                let allocated_new_array = self.ensure_capacity(i)?;

                if allocated_new_array {
                    must_zero = false;
                }
            }

            if must_zero {
                Array::clear(&mut self.buffer, self.length as usize, (i - self.length) as usize)?;
            }

            self.length = i;
        }

        if (count <= 8) {
            let mut byte_count = count as usize;

            loop {
                byte_count = byte_count - 1;

                if byte_count >= 0 {
                    self.buffer[self.position as usize + byte_count] = buffer[offset as usize + byte_count];
                }
            }
        } else {
            Buffer::block_copy(buffer, offset as usize, &mut self.buffer, self.position as usize, count as usize)?;
        }

        self.position = i;

        Ok(())
    }

    fn write_byte(&mut self, value: u8) -> Result<(), Exception> {
        self.ensure_writable()?;
        self.ensure_writable()?;

        if self.position >= self.length {
            let new_length = self.position + 1;
            let mut must_zero = self.position > self.length;

            if new_length >= self.capacity {
                let allocated_new_array = self.ensure_capacity(new_length)?;
                if allocated_new_array {
                    must_zero = false;
                }
            }

            if must_zero {
                Array::clear(&mut self.buffer, self.length as usize, (self.position - self.length) as usize)?;
            }
        }

        let position = self.position;
        self.position = self.position + 1;

        self.buffer[position as usize] = value;

        Ok(())
    }

    fn copy_to(&mut self, destination: &mut dyn Stream, buffer_size: i32) -> Result<(), Exception> {
        //TODO: is memory stream

        StreamHelper::validate_copy_to_arguments(destination, buffer_size)?;
        self.ensure_not_close()?;

        let original_position = self.position;
        let remaining = self.internal_emulated_read(self.length - original_position)?;

        if remaining > 0 {
            destination.write(&self.buffer, original_position, remaining)?;
        }

        Ok(())
    }

    fn write_to(&mut self, stream: &mut dyn Stream) -> Result<(), Exception> {
        self.ensure_not_close()?;

        let slice = self.buffer.as_slice();
        stream.write(slice, self.origin, self.length - self.origin)
    }
}