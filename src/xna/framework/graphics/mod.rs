pub mod packed_vector;
pub mod graphics_adapter;
pub mod blend_state;
pub mod depth_stencil_state;
pub mod rasterizer_state;
pub mod sampler_state;

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
    pub stencil_function: ComparisonFunction,
    pub stencil_pass_operation: StencilOperation,
    pub stencil_fail_operation: StencilOperation,
    pub stencil_depth_fail_operation: StencilOperation,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct DepthStencilState {
    pub depth_enable: bool,
    pub stencil_enable:bool,
    pub depth_function: ComparisonFunction,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub depth_write_mask: bool,
    pub front_face: DepthFace,
    pub back_face: DepthFace,
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
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scale_depth_bias: f32,
    pub scissor_test_enable: bool,
    pub depth_clip_enable: bool,
    pub antialiased_line_enable: bool,
    pub front_counter_clockwise: bool
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
    pub border_color: Color,
    pub comparison_function: ComparisonFunction
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

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct Rational {
    numerator: u32,
    denominator: u32
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum ScanlineOrder {
    #[default]
    Unspecified,
    Progressive,
    UpperField,
    LowerField
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum Scaling {
    #[default]
    Unspecified,
    Centered,
    Stretched,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct DisplayMode {
    width: u32,
    height: u32,
    refresh_rate: Rational,
    format: SurfaceFormat,
    scanline_order: ScanlineOrder,
    scaling: Scaling
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum SurfaceUsage {
    #[default]
    BackBuffer,
    Discard,
    ReadOnly,
    RenderTargetOutput,
    ShaderInput,
    Shared,
    Unordered
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub enum SwapChainFlag {
    #[default]
    NonPreRotated,
    AllowModeSwitch,
    GdiCompatible,
    RestrictedContent,
    RestrictSharedResourceDriver,
    DisplayOnly,
    FrameLatencyWaitableObject,
    ForegroundLayer,
    FullscreenVideo,
    YuvVideo,
    HwProtected,
    AllowTearing,
    RestrictedToAllHolographicDisplays
}

#[derive(Default, PartialEq, Copy, Clone)]
pub struct SwapChain {
    display: DisplayMode,
    sample_count: u32,
    sample_quality: u32,
    usage: SurfaceUsage,
    buffer_count: u32,
    windowed: bool,
    swap_effect: SwapEffect,
    flags: SwapChainFlag,
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

pub enum DataFormat {
    Unknown = 0,
    R32g32b32a32Typeless = 1,
    R32g32b32a32Float = 2,
    R32g32b32a32Uint = 3,
    R32g32b32a32Sint = 4,
    R32g32b32Typeless = 5,
    R32g32b32Float = 6,
    R32g32b32Uint = 7,
    R32g32b32Sint = 8,
    R16g16b16a16Typeless = 9,
    R16g16b16a16Float = 10,
    R16g16b16a16Unorm = 11,
    R16g16b16a16Uint = 12,
    R16g16b16a16Snorm = 13,
    R16g16b16a16Sint = 14,
    R32g32Typeless = 15,
    R32g32Float = 16,
    R32g32Uint = 17,
    R32g32Sint = 18,
    R32g8x24Typeless = 19,
    D32FloatS8x24Uint = 20,
    R32FloatX8x24Typeless = 21,
    X32TypelessG8x24Uint = 22,
    R10g10b10a2Typeless = 23,
    R10g10b10a2Unorm = 24,
    R10g10b10a2Uint = 25,
    R11g11b10Float = 26,
    R8g8b8a8Typeless = 27,
    R8g8b8a8Unorm = 28,
    R8g8b8a8UnormSrgb = 29,
    R8g8b8a8Uint = 30,
    R8g8b8a8Snorm = 31,
    R8g8b8a8Sint = 32,
    R16g16Typeless = 33,
    R16g16Float = 34,
    R16g16Unorm = 35,
    R16g16Uint = 36,
    r16g16_snorm = 37,
    r16g16_sint = 38,
    r32_typeless = 39,
    d32_float = 40,
    r32_float = 41,
    r32_uint = 42,
    r32_sint = 43,
    r24g8_typeless = 44,
    d24_unorm_s8_uint = 45,
    r24_unorm_x8_typeless = 46,
    x24_typeless_g8_uint = 47,
    r8g8_typeless = 48,
    r8g8_unorm = 49,
    r8g8_uint = 50,
    r8g8_snorm = 51,
    r8g8_sint = 52,
    r16_typeless = 53,
    r16_float = 54,
    d16_unorm = 55,
    r16_unorm = 56,
    r16_uint = 57,
    r16_snorm = 58,
    r16_sint = 59,
    r8_typeless = 60,
    r8_unorm = 61,
    r8_uint = 62,
    r8_snorm = 63,
    r8_sint = 64,
    a8_unorm = 65,
    r1_unorm = 66,
    r9g9b9e5_sharedexp = 67,
    r8g8_b8g8_unorm = 68,
    g8r8_g8b8_unorm = 69,
    bc1_typeless = 70,
    bc1_unorm = 71,
    bc1_unorm_srgb = 72,
    bc2_typeless = 73,
    bc2_unorm = 74,
    bc2_unorm_srgb = 75,
    bc3_typeless = 76,
    bc3_unorm = 77,
    bc3_unorm_srgb = 78,
    bc4_typeless = 79,
    bc4_unorm = 80,
    bc4_snorm = 81,
    bc5_typeless = 82,
    bc5_unorm = 83,
    bc5_snorm = 84,
    b5g6r5_unorm = 85,
    b5g5r5a1_unorm = 86,
    b8g8r8a8_unorm = 87,
    b8g8r8x8_unorm = 88,
    r10g10b10_xr_bias_a2_unorm = 89,
    b8g8r8a8_typeless = 90,
    b8g8r8a8_unorm_srgb = 91,
    b8g8r8x8_typeless = 92,
    b8g8r8x8_unorm_srgb = 93,
    bc6h_typeless = 94,
    bc6h_uf16 = 95,
    bc6h_sf16 = 96,
    bc7_typeless = 97,
    bc7_unorm = 98,
    bc7_unorm_srgb = 99,
    ayuv = 100,
    y410 = 101,
    y416 = 102,
    nv12 = 103,
    p010 = 104,
    p016 = 105,
    opaque_420 = 106,
    yuy2 = 107,
    y210 = 108,
    y216 = 109,
    nv11 = 110,
    ai44 = 111,
    ia44 = 112,
    p8 = 113,
    a8p8 = 114,
    b4g4r4a4_unorm = 115,
    p208 = 130,
    v208 = 131,
    v408 = 132,
    sampler_feedback_min_mip_opaque = 189,
    sampler_feedback_mip_region_used_opaque = 190,
    force_uint = 0xffffffff
}