pub mod packed_vector;
pub mod graphics_adapter;
pub mod blend_state;
pub mod depth_stencil_state;
pub mod rasterizer_state;

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

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum StencilOperation
{
    #[default]
    Keep,
    Zero,
    Replace,
    IncrementSaturation,
    DecrementSaturation,
    Invert,
    Increment,
    Decrement,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum ComparisonFunction {
    #[default]
    Never,
    Less,
    Equal,
    LessEquals,
    Greater,
    NotEqual,
    GreaterEqual,
    Always
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct DepthStencilStateCounterClockWise {
    pub depth_buffer_fail: StencilOperation,
    pub stencil_fail: StencilOperation,
    pub stencil_function: ComparisonFunction,
    pub stencil_pass: StencilOperation,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct DepthStencilStateStencil {
    pub enable: bool,
    pub fail: StencilOperation,
    pub function: ComparisonFunction,
    pub mask: i32,
    pub write_mask: i32,
    pub pass: StencilOperation,
    pub depth_buffer_fail: StencilOperation,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct DepthStencilState {
    pub counter_clock_wise: DepthStencilStateCounterClockWise,
    pub stencil: DepthStencilStateStencil,
    pub depth_buffer_enable: bool,
    pub depth_buffer_write_enable: bool,
    pub depth_buffer_function: ComparisonFunction,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum CullMode {
    #[default]
    None,
    CullClockwiseFace,
    CullCounterClockwiseFace,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum FillMode {
    #[default]
    WireFrame,
    Solid
}

#[derive(Default, PartialEq, Copy, Clone)]
pub struct RasterizerState {
    pub cull_mode: CullMode,
    pub fill_mode: FillMode,
    pub multi_sample_anti_alias: bool,
    pub depth_bias: f32,
    pub slope_scale_depth_bias: f32,
    pub scissor_test_enable: bool,
    pub depth_clip_enable: bool
}

#[derive(Default)]
pub struct GraphicsDevice {
    adapter: GraphicsAdapter,
    blend_state: BlendState,
    depth_stencil_state: DepthStencilState,
    rasterizer_state: RasterizerState
}