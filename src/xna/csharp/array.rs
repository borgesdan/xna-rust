use crate::xna::csharp::{Array, Exception};

impl Array {
    pub fn clear<T>(array: &mut [T], index: usize, length: usize)
        -> Result<(), Exception> where T : Default, T : Clone {
        let mut slice = &mut array[index.. length];
        slice.fill(T::default());

        Ok(())
    }
}