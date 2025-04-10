use crate::xna::framework::Vector4;

pub mod packed_vector;

pub trait IPackedVector {
    fn to_vector4(&self) -> Vector4;
}

pub struct PackUtils {}

#[derive(Default)]
pub struct Alpha8 {
    packed_value: u8,
}

#[derive(Default)]
pub struct Bgr565 {
    packed_value: u16,
}


