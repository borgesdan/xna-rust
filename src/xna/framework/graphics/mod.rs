use crate::xna::framework::{Rectangle, Vector4};
pub mod packed_vector;
pub mod graphics_adapter;

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

#[derive(Default)]
pub struct GraphicsAdapter {
    pub index: u32,
    pub description: String,
    pub device_id: u32,
    pub device_name: String,
    pub is_default: bool,
    pub monitor_handle: isize,
    pub revision: u32,
    pub sub_system_id: u32,
    pub vendor_id: u32,
}


