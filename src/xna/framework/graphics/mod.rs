pub mod packed_vector;
pub mod graphics_adapter;
pub mod blend_state;
pub mod depth_stencil_state;
pub mod rasterizer_state;
pub mod sampler_state;
pub mod swap_chain;
pub mod graphics_device;
pub mod display_mode_collection;

use crate::xna::framework::{Color, Rectangle, Vector4};
#[cfg(target_os = "windows")]
use crate::xna::platform::windows::WindowsGraphicsAdapter;
#[cfg(target_os = "windows")]
use crate::xna::platform::windows::WindowsGraphicsAdapterOutput;
#[cfg(target_os = "windows")]
use crate::xna::platform::windows::WindowsGraphicsDevice;
#[cfg(target_os = "windows")]
use crate::xna::platform::windows::WindowsPresentationParameters;
#[cfg(target_os = "windows")]
use crate::xna::platform::windows::WindowsRenderTarget2D;
#[cfg(target_os = "windows")]
use crate::xna::platform::windows::WindowsTexture2D;

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

#[derive(Default, Eq, PartialEq, Clone)]
pub struct GraphicsAdapterOutput {
    pub device_name: String,
    pub desktop_coordinates: Rectangle,
    pub attached_to_desktop: bool,
    pub display_mode_collection: DisplayModeCollection,
    pub current_display_mode: Option<DisplayMode>,

    #[cfg(target_os = "windows")]
    pub platform: WindowsGraphicsAdapterOutput
}

#[derive(Default, Eq, PartialEq, Clone)]
pub struct GraphicsAdapter {
    pub index: u32,
    pub description: String,
    pub device_id: u32,
    pub is_default: bool,
    pub revision: u32,
    pub sub_system_id: u32,
    pub vendor_id: u32,
    pub outputs: Vec<GraphicsAdapterOutput>,
    pub current_output: Option<GraphicsAdapterOutput>,

    #[cfg(target_os = "windows")]
    pub platform: WindowsGraphicsAdapter
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
    pub back_buffer_width: u32,
    pub back_buffer_height: u32,
    pub back_buffer_format: SurfaceFormat,
    pub is_full_screen: bool,
    pub multi_sample_count: u32,
    pub presentation_interval: PresentInterval,
    pub depth_stencil_format: DepthFormat,
    pub presentation_swap_effect: SwapEffect,

    #[cfg(target_os = "windows")]
    pub platform: WindowsPresentationParameters
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

#[derive(Default, PartialEq, Clone)]
pub struct Texture2D {
    pub width: u32,
    pub height: u32,
    pub format: SurfaceFormat,

    #[cfg(target_os = "windows")]
    pub platform: WindowsTexture2D
}

#[derive(Default, PartialEq, Clone)]
pub struct RenderTarget2D {
    pub texture: Texture2D,

    #[cfg(target_os = "windows")]
    pub platform: WindowsRenderTarget2D
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
pub enum DisplayModeScaling {
    #[default]
    Unspecified,
    Centered,
    Stretched,
}

#[derive(Default, Eq, PartialEq, Copy, Clone)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate_numerator: u32,
    pub refresh_rate_denominator: u32,
    pub format: SurfaceFormat,
    pub scanline_order: ScanlineOrder,
    pub scaling: DisplayModeScaling
}

#[derive(Default, Eq, PartialEq, Clone)]
pub struct DisplayModeCollection {
    pub display_modes: Vec<DisplayMode>,
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
    pub display: DisplayMode,
    pub sample_count: u32,
    pub sample_quality: u32,
    pub usage: SurfaceUsage,
    pub buffer_count: u32,
    pub windowed: bool,
    pub swap_effect: SwapEffect,
    pub flags: SwapChainFlag,
}

#[derive(Default, PartialEq, Clone)]
pub struct GraphicsDevice {
    pub adapter: GraphicsAdapter,
    pub blend_state: BlendState,
    pub depth_stencil_state: DepthStencilState,
    pub rasterizer_state: RasterizerState,
    pub sampler_state_collection: SamplerStateCollection,
    pub presentation_parameters: PresentationParameters,
    pub viewport: Viewport,
    pub render_target: RenderTarget2D,
    pub swap_chain: SwapChain,

    #[cfg(target_os = "windows")]
    pub platform: WindowsGraphicsDevice
}
