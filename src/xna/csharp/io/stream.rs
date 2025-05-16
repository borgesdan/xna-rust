use crate::xna::csharp::Exception;
use crate::xna::csharp::io::{Stream, StreamHelper};
impl StreamHelper {
    pub fn copy_to(source: &mut dyn Stream, destination: &mut dyn Stream, buffer_size: i32) -> Result<(), Exception> {
        Self::validate_copy_to_arguments(destination, buffer_size)?;

        if !source.get_can_read()? {
            return Err(Exception::new("Unreadable source stream", None));
        }

        let mut buffer = vec![0u8; buffer_size as usize];
        let mut slice = buffer.as_mut_slice();

        let mut bytes_read = 0;

        loop {
            bytes_read = source.read(&mut slice, 0, buffer_size)?;

            if bytes_read != 0 {
                destination.write(slice, 0, bytes_read)?;
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn get_copy_buffer_size(&self, source: &dyn Stream) -> Result<i32, Exception> {
        let default_copy_buffer_size = 81920i32;
        let mut buffer_size = default_copy_buffer_size;

        if source.get_can_seek()? {
            let length = source.get_length()?;
            let position = source.get_position()?;

            if length <= position {
                buffer_size = 1;
            } else {
                let remaining = length - position;

                if remaining > 0 {
                    buffer_size = std::cmp::min(buffer_size as i64, remaining) as i32;
                }
            }
        }

        Ok(buffer_size)
    }

    pub fn read(source: &mut dyn Stream, buffer: &mut [u8])-> Result<i32, Exception> {
        let num_read = source.read(buffer, 0, buffer.len() as i32)?;

        if num_read as usize > buffer.len(){
            return Err(Exception::new("Stream is too long.", None));
        }

        Ok(num_read)
    }

    pub fn read_byte(source: &mut dyn Stream) -> Result<i32, Exception> {
        let mut one_byte_array = [0u8;1];
        let r = source.read(&mut one_byte_array, 0, 1)?;

        Ok(if r == 0 { -1 } else { one_byte_array[0] as i32 })
    }

    pub fn read_exactly(source: &mut dyn Stream, buffer: &mut [u8], offset: i32, count: i32) -> Result<(), Exception> {
        Self::validate_buffer_arguments(&buffer, offset.clone(), count.clone())?;
        let slice = &mut buffer[offset as usize..(offset + count) as usize];

        _ = Self::read_at_least_core(source, slice, count, true)?;

        Ok(())
    }

    pub fn read_at_least(source: &mut dyn Stream, buffer: &mut [u8], minimum_bytes: i32, throw_on_end_of_stream: bool) -> Result<(i32), Exception> {
        Self::validate_read_at_least_arguments(buffer.len().clone() as i32, minimum_bytes)?;

        Self::read_at_least_core(source, buffer, minimum_bytes, throw_on_end_of_stream)
    }

    pub fn read_at_least_core(source: &mut dyn Stream, buffer: &mut [u8], minimum_bytes: i32, throw_on_end_of_stream: bool) -> Result<(i32), Exception> {
        let mut total_read = 0;

        while total_read < minimum_bytes {
            let slice = &mut buffer[total_read as usize..];
            let read = Self::read(source, slice)?;

            if read == 0 {
                if throw_on_end_of_stream {
                    return Err(Exception::new("End of stream", None));
                }

                return Ok(total_read);
            }

            total_read = total_read + read;
        }

        Ok(total_read)
    }

    pub fn validate_buffer_arguments(buffer: &[u8], offset: i32, count: i32) -> Result<(), Exception> {
        if offset < 0 {
            return Err(Exception::new("Offset cannot be less than 0", None));
        }

        if count as usize > buffer.len() - offset as usize {
            return Err(Exception::new("Count cannot be less than buffer.len() - offset", None));
        }

        Ok(())
    }

    pub fn validate_read_at_least_arguments(buffer_length: i32, minimum_bytes: i32) -> Result<(), Exception> {
        if minimum_bytes < 0 {
            return Err(Exception::new("minimum_bytes cannot be less than 0", None));
        }

        if buffer_length < minimum_bytes {
            return Err(Exception::new("buffer_length cannot be less than minimum_bytes", None));
        }

        Ok(())
    }

    pub fn validate_copy_to_arguments(destination: &dyn Stream, buffer_size: i32) -> Result<(), Exception> {
        if buffer_size <= 0 {
            return Err(Exception::new("buffer_size cannot be less or equals zero", None));
        }

        if !destination.get_can_write()? {
            if destination.get_can_read()? {
                return Err(Exception::new("Destination is an unwritable stream.", None));
            }

            return Err(Exception::new("Destination is closed.", None));
        }

        Ok(())
    }

    pub fn write(destination: &mut dyn Stream, buffer: &[u8]) -> Result<(), Exception> {
        let _ = destination.write(buffer, 0, buffer.len() as i32)?;
        Ok(())
    }

    pub fn write_byte(destination: &mut dyn Stream, value: u8) -> Result<(), Exception> {
        let buffer = [value, 1];
        _ = destination.write(&buffer, 0, 1)?;

        Ok(())
    }
}