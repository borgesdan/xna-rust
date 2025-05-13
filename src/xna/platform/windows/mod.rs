pub mod depth_stencil_state;
pub mod graphics_device;
pub mod blend_state;
pub mod rasterizer_state;
pub mod swap_chain;
mod render_target_2d;
pub mod step_timer;
pub mod game;
pub mod game_window;
pub mod graphics_device_manager;
pub mod graphics_adapter;
pub mod screen;
pub mod system_information;

use crate::xna::csharp::{Exception, ExceptionConverter};
use crate::xna::framework::graphics::{Blend, BlendFunction, ColorWriteChannels, ComparisonFunction, CullMode, DepthFace, DepthStencilState, DisplayMode, DisplayModeScaling, FillMode, IPackedVector, RasterizerState, SamplerState, ScanlineOrder, StencilOperation, SurfaceFormat, SurfaceUsage, SwapChain, SwapChainFlag, SwapEffect, TextureAddressMode, TextureFilter};
use windows::core::{Error, BOOL};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL;
use windows::Win32::Graphics::Direct3D11::{ID3D11BlendState, ID3D11DepthStencilState, ID3D11Device, ID3D11DeviceContext, ID3D11RasterizerState, ID3D11RenderTargetView, ID3D11SamplerState, ID3D11Texture2D, D3D11_BLEND, D3D11_BLEND_BLEND_FACTOR, D3D11_BLEND_DEST_ALPHA, D3D11_BLEND_DEST_COLOR, D3D11_BLEND_INV_BLEND_FACTOR, D3D11_BLEND_INV_DEST_ALPHA, D3D11_BLEND_INV_DEST_COLOR, D3D11_BLEND_INV_SRC1_ALPHA, D3D11_BLEND_INV_SRC1_COLOR, D3D11_BLEND_INV_SRC_ALPHA, D3D11_BLEND_INV_SRC_COLOR, D3D11_BLEND_ONE, D3D11_BLEND_OP, D3D11_BLEND_OP_ADD, D3D11_BLEND_OP_MAX, D3D11_BLEND_OP_MIN, D3D11_BLEND_OP_REV_SUBTRACT, D3D11_BLEND_OP_SUBTRACT, D3D11_BLEND_SRC1_ALPHA, D3D11_BLEND_SRC1_COLOR, D3D11_BLEND_SRC_ALPHA, D3D11_BLEND_SRC_ALPHA_SAT, D3D11_BLEND_SRC_COLOR, D3D11_BLEND_ZERO, D3D11_COLOR_WRITE_ENABLE, D3D11_COLOR_WRITE_ENABLE_ALL, D3D11_COLOR_WRITE_ENABLE_ALPHA, D3D11_COLOR_WRITE_ENABLE_BLUE, D3D11_COLOR_WRITE_ENABLE_GREEN, D3D11_COLOR_WRITE_ENABLE_RED, D3D11_COMPARISON_ALWAYS, D3D11_COMPARISON_EQUAL, D3D11_COMPARISON_FUNC, D3D11_COMPARISON_GREATER, D3D11_COMPARISON_GREATER_EQUAL, D3D11_COMPARISON_LESS, D3D11_COMPARISON_LESS_EQUAL, D3D11_COMPARISON_NEVER, D3D11_COMPARISON_NOT_EQUAL, D3D11_CULL_BACK, D3D11_CULL_FRONT, D3D11_CULL_MODE, D3D11_CULL_NONE, D3D11_DEPTH_STENCILOP_DESC, D3D11_DEPTH_STENCIL_DESC, D3D11_DEPTH_WRITE_MASK, D3D11_FILL_MODE, D3D11_FILL_SOLID, D3D11_FILL_WIREFRAME, D3D11_FILTER, D3D11_FILTER_ANISOTROPIC, D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT, D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR, D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT, D3D11_FILTER_MIN_MAG_MIP_LINEAR, D3D11_FILTER_MIN_MAG_MIP_POINT, D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR, D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT, D3D11_RASTERIZER_DESC, D3D11_SAMPLER_DESC, D3D11_STENCIL_OP, D3D11_STENCIL_OP_DECR, D3D11_STENCIL_OP_DECR_SAT, D3D11_STENCIL_OP_INCR, D3D11_STENCIL_OP_INCR_SAT, D3D11_STENCIL_OP_INVERT, D3D11_STENCIL_OP_KEEP, D3D11_STENCIL_OP_REPLACE, D3D11_STENCIL_OP_ZERO, D3D11_TEXTURE_ADDRESS_BORDER, D3D11_TEXTURE_ADDRESS_CLAMP, D3D11_TEXTURE_ADDRESS_MIRROR, D3D11_TEXTURE_ADDRESS_MIRROR_ONCE, D3D11_TEXTURE_ADDRESS_MODE, D3D11_TEXTURE_ADDRESS_WRAP};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_FORMAT_UNKNOWN, DXGI_MODE_DESC, DXGI_MODE_SCALING, DXGI_MODE_SCALING_CENTERED, DXGI_MODE_SCALING_STRETCHED, DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER, DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST, DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST, DXGI_RATIONAL, DXGI_SAMPLE_DESC};
use windows::Win32::Graphics::Dxgi::{IDXGIAdapter, IDXGIFactory, IDXGIOutput, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING, DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY, DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER, DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT, DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO, DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE, DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED, DXGI_SWAP_CHAIN_FLAG_NONPREROTATED, DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT, DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS, DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER, DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO, DXGI_SWAP_EFFECT, DXGI_SWAP_EFFECT_DISCARD, DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL, DXGI_SWAP_EFFECT_SEQUENTIAL, DXGI_USAGE, DXGI_USAGE_BACK_BUFFER, DXGI_USAGE_DISCARD_ON_PRESENT, DXGI_USAGE_READ_ONLY, DXGI_USAGE_RENDER_TARGET_OUTPUT, DXGI_USAGE_SHADER_INPUT, DXGI_USAGE_SHARED, DXGI_USAGE_UNORDERED_ACCESS};
use windows::Win32::Graphics::Gdi::HMONITOR;

impl<T> ExceptionConverter<T> for Result<T, Error> {
    fn unwrap_or_exception(self, message: &str) -> Result<T, Exception> {
        if self.is_ok() {
            return Ok(self.unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let inner = Exception::from(error.clone());
        let h_result = error.code();
        let exception = Exception::create(message, h_result.0 as isize,  Some(inner));

        Err(exception)
    }
}

#[derive(Default, Eq, PartialEq, Clone, Copy, Debug)]
pub struct StepTimer {
    frequency: i64,
    last_time: i64,
    max_delta: u64,

    elapsed_ticks: u64,
    total_ticks: u64,
    left_over_ticks: u64,

    frame_count: u32,
    frames_per_second: u32,
    frames_this_second: u32,
    second_counter: u64,

    pub target_elapsed_ticks: u64,
    pub is_fixed_time_step: bool,
}

#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct WindowsGraphicsAdapterOutput {
    output: Option<IDXGIOutput>,
}

#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct WindowsGraphicsAdapter{
    factory: Option<IDXGIFactory>,
    adapter: Option<IDXGIAdapter>,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Copy)]
pub struct WindowsGameWindow {
    hwnd: HWND,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub struct WindowsPresentationParameters {
    pub hwnd: HWND,
}

#[derive(Default, PartialEq, Clone, Debug, Eq)]
pub struct WindowsGraphicsDevice {
    device: Option<ID3D11Device>,
    context: Option<ID3D11DeviceContext>,
    factory: Option<IDXGIFactory>,
    feature_level: D3D_FEATURE_LEVEL,
    blend_state: Option<ID3D11BlendState>,
    rasterizer_state: Option<ID3D11RasterizerState>,
    swap_chain: Option<IDXGISwapChain>,
    depth_stencil_state: Option<ID3D11DepthStencilState>,
    render_target: Option<ID3D11RenderTargetView>,
    sampler_state_collection: Vec<Option<ID3D11SamplerState>>,

    is_initialized: bool
}

#[derive(Default, PartialEq, Clone, Eq, Debug, Copy)]
pub struct WindowsGame {
    pub is_running: bool,
    pub step_timer: StepTimer,
}

#[derive(Default, Clone, PartialEq, Debug, Eq)]
pub struct WindowsRenderTarget2D {
    pub view: Option<ID3D11RenderTargetView>,
    pub texture: Option<ID3D11Texture2D>
}

#[derive(Default, Clone, PartialEq, Debug, Eq, Copy)]
pub struct WindowsScreen {
    pub h_monitor: HMONITOR,
}

impl From<Error> for Exception {
    fn from(value: Error) -> Self {
        let message = value.message();
        let code = value.code();

        Exception::create(message.as_str(), code.0 as isize, None)
    }
}

impl From<Blend> for D3D11_BLEND {
    fn from(value: Blend) -> Self {
        match value {
            Blend::Zero => D3D11_BLEND_ZERO,
            Blend::One => D3D11_BLEND_ONE,
            Blend::SourceColor => D3D11_BLEND_SRC_COLOR,
            Blend::InverseSourceColor => D3D11_BLEND_INV_SRC_COLOR,
            Blend::SourceAlpha => D3D11_BLEND_SRC_ALPHA,
            Blend::InverseSourceAlpha => D3D11_BLEND_INV_SRC_ALPHA,
            Blend::DestinationAlpha => D3D11_BLEND_DEST_ALPHA,
            Blend::InverseDestinationAlpha => D3D11_BLEND_INV_DEST_ALPHA,
            Blend::DestinationColor => D3D11_BLEND_DEST_COLOR,
            Blend::InverseDestinationColor => D3D11_BLEND_INV_DEST_COLOR,
            Blend::SourceAlphaSaturation => D3D11_BLEND_SRC_ALPHA_SAT,
            Blend::BlendFactor => D3D11_BLEND_BLEND_FACTOR,
            Blend::InverseBlendFactor => D3D11_BLEND_INV_BLEND_FACTOR,
            Blend::Source1Color => D3D11_BLEND_SRC1_COLOR,
            Blend::InverseSource1Color => D3D11_BLEND_INV_SRC1_COLOR,
            Blend::Source1Alpha => D3D11_BLEND_SRC1_ALPHA,
            Blend::InverseSource1Alpha => D3D11_BLEND_INV_SRC1_ALPHA,
        }
    }
}

impl From<BlendFunction> for D3D11_BLEND_OP {
    fn from(value: BlendFunction) -> Self {
        match value {
            BlendFunction::Add => D3D11_BLEND_OP_ADD,
            BlendFunction::Max => D3D11_BLEND_OP_MAX,
            BlendFunction::Min => D3D11_BLEND_OP_MIN,
            BlendFunction::ReverseSubtract => D3D11_BLEND_OP_REV_SUBTRACT,
            BlendFunction::Subtract => D3D11_BLEND_OP_SUBTRACT,
        }
    }
}

impl From<ColorWriteChannels> for  D3D11_COLOR_WRITE_ENABLE {
    fn from(value: ColorWriteChannels) -> Self {
        match value {
            ColorWriteChannels::Red => D3D11_COLOR_WRITE_ENABLE_RED,
            ColorWriteChannels::Green => D3D11_COLOR_WRITE_ENABLE_GREEN,
            ColorWriteChannels::Blue => D3D11_COLOR_WRITE_ENABLE_BLUE,
            ColorWriteChannels::Alpha => D3D11_COLOR_WRITE_ENABLE_ALPHA,
            ColorWriteChannels::All => D3D11_COLOR_WRITE_ENABLE_ALL,
        }
    }
}

impl From<ComparisonFunction > for D3D11_COMPARISON_FUNC {
    fn from(value: ComparisonFunction) -> Self {
        match value {
            ComparisonFunction::Never => D3D11_COMPARISON_NEVER,
            ComparisonFunction::Less => D3D11_COMPARISON_LESS,
            ComparisonFunction::Equal => D3D11_COMPARISON_EQUAL,
            ComparisonFunction::LessEquals => D3D11_COMPARISON_LESS_EQUAL,
            ComparisonFunction::Greater => D3D11_COMPARISON_GREATER,
            ComparisonFunction::NotEqual => D3D11_COMPARISON_NOT_EQUAL,
            ComparisonFunction::GreaterEqual => D3D11_COMPARISON_GREATER_EQUAL,
            ComparisonFunction::Always => D3D11_COMPARISON_ALWAYS,
        }
    }
}

impl From<StencilOperation > for D3D11_STENCIL_OP {
    fn from(value: StencilOperation) -> Self {
        match value {
            StencilOperation::Keep => D3D11_STENCIL_OP_KEEP,
            StencilOperation::Zero => D3D11_STENCIL_OP_ZERO,
            StencilOperation::Replace => D3D11_STENCIL_OP_REPLACE,
            StencilOperation::IncrementSaturation => D3D11_STENCIL_OP_INCR_SAT,
            StencilOperation::DecrementSaturation => D3D11_STENCIL_OP_DECR_SAT,
            StencilOperation::Invert => D3D11_STENCIL_OP_INVERT,
            StencilOperation::Increment => D3D11_STENCIL_OP_INCR,
            StencilOperation::Decrement => D3D11_STENCIL_OP_DECR,
        }
    }
}

impl From<CullMode > for D3D11_CULL_MODE {
    fn from(value: CullMode) -> Self {
        match value {
            CullMode::None => D3D11_CULL_NONE,
            CullMode::CullClockwiseFace => D3D11_CULL_FRONT,
            CullMode::CullCounterClockwiseFace => D3D11_CULL_BACK
        }
    }
}

impl From<FillMode > for D3D11_FILL_MODE {
    fn from(value: FillMode) -> Self {
        match value {
            FillMode::WireFrame => D3D11_FILL_WIREFRAME,
            FillMode::Solid => D3D11_FILL_SOLID,
        }
    }
}

impl From<TextureAddressMode > for D3D11_TEXTURE_ADDRESS_MODE {
    fn from(value: TextureAddressMode) -> Self {
        match value {
            TextureAddressMode::Wrap => D3D11_TEXTURE_ADDRESS_WRAP,
            TextureAddressMode::Mirror => D3D11_TEXTURE_ADDRESS_MIRROR,
            TextureAddressMode::Clamp => D3D11_TEXTURE_ADDRESS_CLAMP,
            TextureAddressMode::Border => D3D11_TEXTURE_ADDRESS_BORDER,
            TextureAddressMode::MirrorOnce => D3D11_TEXTURE_ADDRESS_MIRROR_ONCE,
        }
    }
}

impl From<TextureFilter > for D3D11_FILTER {
    fn from(value: TextureFilter) -> Self {
        match value {
            TextureFilter::Linear => D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            TextureFilter::Point => D3D11_FILTER_MIN_MAG_MIP_POINT,
            TextureFilter::Anisotropic => D3D11_FILTER_ANISOTROPIC,
            TextureFilter::LinearMipPoint => D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT,
            TextureFilter::PointMipLinear => D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR,
            TextureFilter::MinLinearMagPointMipLinear => D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
            TextureFilter::MinLinearMagPointMipPoint => D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT,
            TextureFilter::MinPointMagLinearMipLinear => D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT,
            TextureFilter::MinPointMagLinearMipPoint => D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT,
        }
    }
}

impl From<DisplayMode > for DXGI_MODE_DESC {
    fn from(value: DisplayMode) -> Self {
        DXGI_MODE_DESC {
            Height: value.height,
            Width: value.width,
            RefreshRate: DXGI_RATIONAL {
                Denominator: value.refresh_rate_denominator,
                Numerator: value.refresh_rate_numerator,
            },
            Format: DXGI_FORMAT::from(value.format),
            Scaling: DXGI_MODE_SCALING::from(value.scaling),
            ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER::from(value.scanline_order),
        }
    }
}

impl From<SurfaceFormat > for DXGI_FORMAT {
    fn from(value: SurfaceFormat) -> Self {
        match value {
            SurfaceFormat::Color => DXGI_FORMAT_R8G8B8A8_UNORM,
            SurfaceFormat::Unknown => DXGI_FORMAT_UNKNOWN,
        }
    }
}

impl From<DisplayModeScaling > for DXGI_MODE_SCALING {
    fn from(value: DisplayModeScaling) -> Self {
        match value {
            DisplayModeScaling::Unspecified => DXGI_MODE_SCALING_UNSPECIFIED,
            DisplayModeScaling::Centered => DXGI_MODE_SCALING_CENTERED,
            DisplayModeScaling::Stretched => DXGI_MODE_SCALING_STRETCHED
        }
    }
}

impl From<ScanlineOrder > for DXGI_MODE_SCANLINE_ORDER {
    fn from(value: ScanlineOrder) -> Self {
        match value {
            ScanlineOrder::Unspecified => DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
            ScanlineOrder::Progressive => DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
            ScanlineOrder::UpperField => DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST,
            ScanlineOrder::LowerField => DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST,
        }
    }
}

impl From<SwapEffect > for DXGI_SWAP_EFFECT {
    fn from(value: SwapEffect) -> Self {
        match value {
            SwapEffect::Discard => DXGI_SWAP_EFFECT_DISCARD,
            SwapEffect::Sequential => DXGI_SWAP_EFFECT_SEQUENTIAL,
            SwapEffect::FlipSequential => DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            SwapEffect::FlipDiscard => DXGI_SWAP_EFFECT_FLIP_DISCARD
        }
    }
}

impl From<SurfaceUsage > for DXGI_USAGE {
    fn from(value: SurfaceUsage) -> Self {
        match value {
            SurfaceUsage::BackBuffer => DXGI_USAGE_BACK_BUFFER,
            SurfaceUsage::Discard => DXGI_USAGE_DISCARD_ON_PRESENT,
            SurfaceUsage::ReadOnly => DXGI_USAGE_READ_ONLY,
            SurfaceUsage::RenderTargetOutput => DXGI_USAGE_RENDER_TARGET_OUTPUT,
            SurfaceUsage::ShaderInput => DXGI_USAGE_SHADER_INPUT,
            SurfaceUsage::Shared => DXGI_USAGE_SHARED,
            SurfaceUsage::Unordered => DXGI_USAGE_UNORDERED_ACCESS,
        }
    }
}

impl From<SwapChainFlag > for DXGI_SWAP_CHAIN_FLAG {
    fn from(value: SwapChainFlag) -> Self {
        match value {
            SwapChainFlag::NonPreRotated => DXGI_SWAP_CHAIN_FLAG_NONPREROTATED,
            SwapChainFlag::AllowModeSwitch =>DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
            SwapChainFlag::GdiCompatible => DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE,
            SwapChainFlag::RestrictedContent => DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT,
            SwapChainFlag::RestrictSharedResourceDriver => DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER,
            SwapChainFlag::DisplayOnly => DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY,
            SwapChainFlag::FrameLatencyWaitableObject => DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT,
            SwapChainFlag::ForegroundLayer => DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER,
            SwapChainFlag::FullscreenVideo => DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO,
            SwapChainFlag::YuvVideo => DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO,
            SwapChainFlag::HwProtected => DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED,
            SwapChainFlag::AllowTearing => DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING,
            SwapChainFlag::RestrictedToAllHolographicDisplays => DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS,
        }
    }
}

impl From<DXGI_FORMAT> for SurfaceFormat{
    fn from(value: DXGI_FORMAT) -> Self {
        match value {
            DXGI_FORMAT_R8G8B8A8_UNORM => SurfaceFormat::Color,
            _ => SurfaceFormat::Unknown,
        }
    }
}

impl From<DXGI_MODE_SCALING> for DisplayModeScaling{
    fn from(value: DXGI_MODE_SCALING) -> Self {
        match value {
            DXGI_MODE_SCALING_STRETCHED => DisplayModeScaling::Stretched,
            DXGI_MODE_SCALING_CENTERED => DisplayModeScaling::Centered,
            _ => DisplayModeScaling::Unspecified
        }
    }
}

impl From<DXGI_MODE_SCANLINE_ORDER> for ScanlineOrder {
    fn from(value: DXGI_MODE_SCANLINE_ORDER) -> Self {
        match value {
            DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST => ScanlineOrder::LowerField,
            DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE => ScanlineOrder::Progressive,
            DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST => ScanlineOrder::UpperField,
            _ => ScanlineOrder::Unspecified
        }
    }
}

impl From<DepthFace> for D3D11_DEPTH_STENCILOP_DESC{
    fn from(value: DepthFace) -> Self {
        D3D11_DEPTH_STENCILOP_DESC {
            StencilFunc: D3D11_COMPARISON_FUNC::from(value.stencil_function),
            StencilPassOp: D3D11_STENCIL_OP::from(value.stencil_pass_operation),
            StencilDepthFailOp: D3D11_STENCIL_OP::from(value.stencil_depth_fail_operation),
            StencilFailOp: D3D11_STENCIL_OP::from(value.stencil_fail_operation),
        }
    }
}

impl From<RasterizerState> for D3D11_RASTERIZER_DESC {
    fn from(value: RasterizerState) -> Self {
        D3D11_RASTERIZER_DESC {
            CullMode: D3D11_CULL_MODE::from(value.cull_mode),
            AntialiasedLineEnable: BOOL::from(value.antialiased_line_enable),
            ScissorEnable: BOOL::from(value.scissor_test_enable),
            SlopeScaledDepthBias: value.slope_scale_depth_bias,
            DepthBias: value.depth_bias,
            MultisampleEnable: BOOL::from(value.multi_sample_anti_alias),
            FillMode: D3D11_FILL_MODE::from(value.fill_mode),
            DepthBiasClamp: value.depth_bias_clamp,
            DepthClipEnable: BOOL::from(value.depth_clip_enable),
            FrontCounterClockwise: BOOL::from(value.front_counter_clockwise),
        }
    }
}

impl From<DepthStencilState> for  D3D11_DEPTH_STENCIL_DESC{
    fn from(value: DepthStencilState) -> Self {
        D3D11_DEPTH_STENCIL_DESC {
            DepthEnable: BOOL::from(value.depth_enable),
            StencilEnable: BOOL::from(value.stencil_enable),
            DepthFunc: D3D11_COMPARISON_FUNC::from(value.depth_function),
            FrontFace: D3D11_DEPTH_STENCILOP_DESC::from(value.front_face),
            BackFace: D3D11_DEPTH_STENCILOP_DESC::from(value.back_face),
            StencilReadMask: value.stencil_read_mask,
            StencilWriteMask: value.stencil_write_mask,
            DepthWriteMask: D3D11_DEPTH_WRITE_MASK(value.depth_write_mask as i32),
        }
    }
}

impl From<SamplerState> for  D3D11_SAMPLER_DESC{
    fn from(value: SamplerState) -> Self {
        let border_color = value.border_color.to_vector4();

        D3D11_SAMPLER_DESC {
            AddressU: D3D11_TEXTURE_ADDRESS_MODE::from(value.address_u),
            AddressV: D3D11_TEXTURE_ADDRESS_MODE::from(value.address_v),
            AddressW: D3D11_TEXTURE_ADDRESS_MODE::from(value.address_w),
            BorderColor: [border_color.x, border_color.y, border_color.z, border_color.w],
            ComparisonFunc: D3D11_COMPARISON_FUNC::from(value.comparison_function),
            Filter: D3D11_FILTER::from(value.filter),
            MaxAnisotropy: value.max_anisotropy,
            MaxLOD: value.max_mip_level,
            MinLOD: value.min_mip_level,
            MipLODBias: value.mip_map_level_of_detail_bias,
        }
    }
}

impl From<SwapChain> for DXGI_SWAP_CHAIN_DESC{
    fn from(value: SwapChain) -> Self {
        DXGI_SWAP_CHAIN_DESC {
            Windowed: BOOL::from(value.windowed),
            BufferCount: value.buffer_count,
            Flags: DXGI_SWAP_CHAIN_FLAG::from(value.flags).0 as u32,
            BufferDesc: DXGI_MODE_DESC::from(value.display),
            SwapEffect: DXGI_SWAP_EFFECT::from(value.swap_effect),
            BufferUsage: DXGI_USAGE::from(value.usage),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: value.sample_count,
                Quality: value.sample_quality,
            },
            ..Default::default()
        }
    }
}