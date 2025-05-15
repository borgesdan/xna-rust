use crate::xna::csharp::{Buffer, Exception};

impl Buffer {
    pub fn block_copy<T>(src: &[T], src_offset: usize, dst: &mut [T], dst_offset: usize, count: usize)
        -> Result<(), Exception> where T: Copy {
        let source = &src[src_offset..count];
        let destination = &mut dst[dst_offset..count];

        destination.copy_from_slice(source);

        Ok(())
    }
}