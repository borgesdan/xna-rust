pub mod packed_vector;
pub mod graphics_adapter;
pub mod blend_state;

use crate::xna::framework::{Color, Rectangle, Vector4};

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

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum Blend {
    #[default]
    Zero,
    One,
    SourceColor,
    InverseSourceColor,
    SourceAlpha,
    InverseSourceAlpha,
    DestinationAlpha,
    InverseDestinationAlpha,
    DestinationColor,
    InverseDestinationColor,
    SourceAlphaSaturation,
    BlendFactor,
    InverseBlendFactor,
    Source1Color,
    InverseSource1Color,
    Source1Alpha,
    InverseSource1Alpha
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum BlendFunction {
    #[default]
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum ColorWriteChannels {
    #[default]
    Red,
    Green,
    Blue,
    Alpha,
    All,
    None,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct BlendRenderTarget {
    pub enabled: bool,
    pub source: Blend,
    pub destination: Blend,
    pub operation: BlendFunction,
    pub source_alpha: Blend,
    pub destination_alpha: Blend,
    pub operation_alpha: BlendFunction,
    pub write_mask: ColorWriteChannels,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct BlendState {
    pub alpha_blend_function: BlendFunction,
    pub alpha_destination_blend: Blend,
    pub alpha_source_blend: Blend,
    pub color_blend_function: BlendFunction,
    pub color_destination_blend: Blend,
    pub color_source_blend: Blend,
    pub blend_factor: Color,
    pub multi_sample_mask: i32,
    pub alpha_to_coverage_enable: bool,
    pub render_targets: [BlendRenderTarget; 8]
}

#[derive(Default)]
pub struct GraphicsDevice {
    adapter: Box<GraphicsAdapter>,
}