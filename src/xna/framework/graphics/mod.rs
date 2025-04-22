pub mod packed_vector;
pub mod graphics_adapter;
pub mod blend_state;
pub mod depth_stencil_state;
pub mod rasterizer_state;
pub mod sampler_state;
pub mod graphics_device;

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
    InverseSource1Alpha,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum BlendFunction {
    #[default]
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
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
    pub blend_factor: Color,
    pub multi_sample_mask: u32,
    pub alpha_to_coverage_enable: bool,
    pub independent_blend_enable: bool,
    pub render_targets: [BlendRenderTarget; 8],
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
    Always,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct DepthFace{
    stencil_function: ComparisonFunction,
    stencil_pass_operation: StencilOperation,
    stencil_fail_operation: StencilOperation,
    stencil_depth_fail_operation: StencilOperation,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct DepthStencilState {
    depth_enable: bool,
    stencil_enable:bool,
    depth_function: ComparisonFunction,
    stencil_read_mask: u8,
    stencil_write_mask: u8,
    depth_write_mask: bool,
    front_face: DepthFace,
    back_face: DepthFace,
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
    Solid,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub struct RasterizerState {
    pub cull_mode: CullMode,
    pub fill_mode: FillMode,
    pub multi_sample_anti_alias: bool,
    pub depth_bias: f32,
    pub slope_scale_depth_bias: f32,
    pub scissor_test_enable: bool,
    pub depth_clip_enable: bool,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum TextureFilter {
    #[default]
    Linear,
    Point,
    Anisotropic,
    LinearMipPoint,
    PointMipLinear,
    MinLinearMagPointMipLinear,
    MinLinearMagPointMipPoint,
    MinPointMagLinearMipLinear,
    MinPointMagLinearMipPoint,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum TextureAddressMode {
    #[default]
    Wrap,
    Mirror,
    Clamp,
    Border,
    MirrorOnce,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub struct SamplerState {
    pub max_anisotropy: u32,
    pub filter: TextureFilter,
    pub address_u: TextureAddressMode,
    pub address_v: TextureAddressMode,
    pub address_w: TextureAddressMode,
    pub mip_map_level_of_detail_bias: f32,
    pub max_mip_level: f32,
    pub min_mip_level: f32,
}

#[derive(Default, PartialEq, Clone)]
pub struct SamplerStateCollection {
    pub samplers: Vec<SamplerState>
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum SurfaceFormat {
    #[default]
    Color,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum PresentInterval {
    #[default]
    Default,
    One,
    Two,
    Immediate
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum DepthFormat {
    #[default]
    None,
    Depth16,
    Depth24,
    Depth24Stencil8
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum SwapEffect {
    Discard,
    Sequential,
    FlipSequential,
    #[default]
    FlipDiscard
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct PresentationParameters {
    pub back_buffer_width: i32,
    pub back_buffer_height: i32,
    pub back_buffer_format: SurfaceFormat,
    pub is_full_screen: bool,
    pub multi_sample_count: i32,
    pub presentation_interval: PresentInterval,
    pub depth_stencil_format: DepthFormat,
    pub presentation_swap_effect: SwapEffect
}

#[derive(Default, PartialEq, Copy, Clone)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32
}

#[derive(Default, PartialEq, Copy, Clone)]
pub struct Texture2D {

}

#[derive(Default, PartialEq, Copy, Clone)]
pub struct RenderTarget {

}

#[derive(Default)]
pub struct GraphicsDevice {
    pub adapter: GraphicsAdapter,
    pub blend_state: BlendState,
    pub depth_stencil_state: DepthStencilState,
    pub rasterizer_state: RasterizerState,
    pub sampler_state_collection: SamplerStateCollection,
    pub presentation_parameters: PresentationParameters,
    pub viewport: Viewport,
    pub render_target: RenderTarget
}