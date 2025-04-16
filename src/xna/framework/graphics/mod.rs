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
    enabled: bool,
    source: Blend,
    destination: Blend,
    operation: BlendFunction,
    source_alpha: Blend,
    destination_alpha: Blend,
    operation_alpha: BlendFunction,
    write_mask: ColorWriteChannels,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct BlendState {
    alpha_blend_function: BlendFunction,
    alpha_destination_blend: Blend,
    alpha_source_blend: Blend,
    color_blend_function: BlendFunction,
    color_destination_blend: Blend,
    color_source_blend: Blend,
    blend_factor: Color,
    multi_sample_mask: i32,
    alpha_to_coverage_enable: bool,
    render_targets: [BlendRenderTarget; 8]
}

#[derive(Default)]
pub struct GraphicsDevice {
    adapter: Box<GraphicsAdapter>,
}
