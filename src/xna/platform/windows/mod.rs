pub mod depth_stencil_state;
pub mod graphics_device;
pub mod blend_state;
pub mod rasterizer_state;
pub mod sampler_state;
pub mod swap_chain;
mod render_target_2d;
mod texture_2d;

use windows::core::BOOL;
use windows::Win32::Foundation::{FALSE, TRUE};
use windows::Win32::Graphics::Direct3D11::{D3D11_BLEND, D3D11_BLEND_BLEND_FACTOR, D3D11_BLEND_DEST_ALPHA, D3D11_BLEND_DEST_COLOR, D3D11_BLEND_INV_BLEND_FACTOR, D3D11_BLEND_INV_DEST_ALPHA, D3D11_BLEND_INV_DEST_COLOR, D3D11_BLEND_INV_SRC1_ALPHA, D3D11_BLEND_INV_SRC1_COLOR, D3D11_BLEND_INV_SRC_ALPHA, D3D11_BLEND_INV_SRC_COLOR, D3D11_BLEND_ONE, D3D11_BLEND_OP, D3D11_BLEND_OP_ADD, D3D11_BLEND_OP_MAX, D3D11_BLEND_OP_MIN, D3D11_BLEND_OP_REV_SUBTRACT, D3D11_BLEND_OP_SUBTRACT, D3D11_BLEND_SRC1_ALPHA, D3D11_BLEND_SRC1_COLOR, D3D11_BLEND_SRC_ALPHA, D3D11_BLEND_SRC_ALPHA_SAT, D3D11_BLEND_SRC_COLOR, D3D11_BLEND_ZERO, D3D11_COLOR_WRITE_ENABLE, D3D11_COLOR_WRITE_ENABLE_ALL, D3D11_COLOR_WRITE_ENABLE_ALPHA, D3D11_COLOR_WRITE_ENABLE_BLUE, D3D11_COLOR_WRITE_ENABLE_GREEN, D3D11_COLOR_WRITE_ENABLE_RED, D3D11_COMPARISON_ALWAYS, D3D11_COMPARISON_EQUAL, D3D11_COMPARISON_FUNC, D3D11_COMPARISON_GREATER, D3D11_COMPARISON_GREATER_EQUAL, D3D11_COMPARISON_LESS, D3D11_COMPARISON_LESS_EQUAL, D3D11_COMPARISON_NEVER, D3D11_COMPARISON_NOT_EQUAL, D3D11_CULL_BACK, D3D11_CULL_FRONT, D3D11_CULL_MODE, D3D11_CULL_NONE, D3D11_FILL_MODE, D3D11_FILL_SOLID, D3D11_FILL_WIREFRAME, D3D11_FILTER, D3D11_FILTER_ANISOTROPIC, D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT, D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR, D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT, D3D11_FILTER_MIN_MAG_MIP_LINEAR, D3D11_FILTER_MIN_MAG_MIP_POINT, D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR, D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT, D3D11_STENCIL_OP, D3D11_STENCIL_OP_DECR, D3D11_STENCIL_OP_DECR_SAT, D3D11_STENCIL_OP_INCR, D3D11_STENCIL_OP_INCR_SAT, D3D11_STENCIL_OP_INVERT, D3D11_STENCIL_OP_KEEP, D3D11_STENCIL_OP_REPLACE, D3D11_STENCIL_OP_ZERO, D3D11_TEXTURE_ADDRESS_BORDER, D3D11_TEXTURE_ADDRESS_CLAMP, D3D11_TEXTURE_ADDRESS_MIRROR, D3D11_TEXTURE_ADDRESS_MIRROR_ONCE, D3D11_TEXTURE_ADDRESS_MODE, D3D11_TEXTURE_ADDRESS_WRAP};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_MODE_DESC, DXGI_MODE_SCALING, DXGI_MODE_SCALING_CENTERED, DXGI_MODE_SCALING_STRETCHED, DXGI_MODE_SCALING_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER, DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST, DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE, DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST, DXGI_RATIONAL};
use windows::Win32::Graphics::Dxgi::{DXGI_SCALING, DXGI_SURFACE_DESC, DXGI_SWAP_CHAIN_DESC, DXGI_SWAP_CHAIN_FLAG, DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING, DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY, DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER, DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT, DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO, DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE, DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED, DXGI_SWAP_CHAIN_FLAG_NONPREROTATED, DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT, DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS, DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER, DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO, DXGI_SWAP_EFFECT, DXGI_SWAP_EFFECT_DISCARD, DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL, DXGI_SWAP_EFFECT_SEQUENTIAL, DXGI_USAGE, DXGI_USAGE_BACK_BUFFER, DXGI_USAGE_DISCARD_ON_PRESENT, DXGI_USAGE_READ_ONLY, DXGI_USAGE_RENDER_TARGET_OUTPUT, DXGI_USAGE_SHADER_INPUT, DXGI_USAGE_SHARED, DXGI_USAGE_UNORDERED_ACCESS};
use crate::xna::framework::graphics::{Blend, BlendFunction, ColorWriteChannels, ComparisonFunction, CullMode, DisplayMode, DisplayModeScaling, FillMode, ScanlineOrder, StencilOperation, SurfaceFormat, SurfaceUsage, SwapChainFlag, SwapEffect, TextureAddressMode, TextureFilter};

pub fn bool_to_win_bool(bool: bool) -> BOOL {
    if bool { TRUE } else { FALSE }
}

impl Blend {
    pub fn to_dx(&self) -> D3D11_BLEND {
        match self {
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

impl BlendFunction {
    pub fn to_dx(&self) -> D3D11_BLEND_OP {
        match self {
            BlendFunction::Add => D3D11_BLEND_OP_ADD,
            BlendFunction::Max => D3D11_BLEND_OP_MAX,
            BlendFunction::Min => D3D11_BLEND_OP_MIN,
            BlendFunction::ReverseSubtract => D3D11_BLEND_OP_REV_SUBTRACT,
            BlendFunction::Subtract => D3D11_BLEND_OP_SUBTRACT,
        }
    }
}

impl ColorWriteChannels {
    pub fn to_dx(&self) -> D3D11_COLOR_WRITE_ENABLE {
        match self {
            ColorWriteChannels::Red => D3D11_COLOR_WRITE_ENABLE_RED,
            ColorWriteChannels::Green => D3D11_COLOR_WRITE_ENABLE_GREEN,
            ColorWriteChannels::Blue => D3D11_COLOR_WRITE_ENABLE_BLUE,
            ColorWriteChannels::Alpha => D3D11_COLOR_WRITE_ENABLE_ALPHA,
            ColorWriteChannels::All => D3D11_COLOR_WRITE_ENABLE_ALL,
        }
    }
}

impl ComparisonFunction {
    pub fn to_dx(&self) -> D3D11_COMPARISON_FUNC {
        match self {
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

impl StencilOperation {
    pub fn to_dx(&self) -> D3D11_STENCIL_OP {
        match self {
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

impl CullMode {
    pub fn to_dx(&self) -> D3D11_CULL_MODE {
        match self {
            CullMode::None => D3D11_CULL_NONE,
            CullMode::CullClockwiseFace => D3D11_CULL_FRONT,
            CullMode::CullCounterClockwiseFace => D3D11_CULL_BACK
        }
    }
}

impl FillMode {
    pub fn to_dx(&self) -> D3D11_FILL_MODE {
        match self {
            FillMode::WireFrame => D3D11_FILL_WIREFRAME,
            FillMode::Solid => D3D11_FILL_SOLID,
        }
    }
}

impl TextureAddressMode {
    pub fn to_dx(&self) -> D3D11_TEXTURE_ADDRESS_MODE {
        match self {
            TextureAddressMode::Wrap => D3D11_TEXTURE_ADDRESS_WRAP,
            TextureAddressMode::Mirror => D3D11_TEXTURE_ADDRESS_MIRROR,
            TextureAddressMode::Clamp => D3D11_TEXTURE_ADDRESS_CLAMP,
            TextureAddressMode::Border => D3D11_TEXTURE_ADDRESS_BORDER,
            TextureAddressMode::MirrorOnce => D3D11_TEXTURE_ADDRESS_MIRROR_ONCE,
        }
    }
}

impl TextureFilter {
    pub fn to_dx(&self) -> D3D11_FILTER {
        match self {
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

impl DisplayMode {
    pub fn to_dx(&self) -> DXGI_MODE_DESC {
        DXGI_MODE_DESC {
            Height: self.height,
            Width: self.width,
            RefreshRate: DXGI_RATIONAL {
                Denominator: self.refresh_rate_denominator,
                Numerator: self.refresh_rate_numerator,
            },
            Format: self.format.to_dx(),
            Scaling: self.scaling.to_dx(),
            ScanlineOrdering: self.scanline_order.to_dx(),
        }
    }
}

impl SurfaceFormat {
    pub fn to_dx(&self) -> DXGI_FORMAT {
        match self {
            SurfaceFormat::Color => DXGI_FORMAT_R8G8B8A8_UNORM,
        }
    }
}

impl DisplayModeScaling {
    pub fn to_dx(&self) -> DXGI_MODE_SCALING {
        match self {
            DisplayModeScaling::Unspecified => DXGI_MODE_SCALING_UNSPECIFIED,
            DisplayModeScaling::Centered => DXGI_MODE_SCALING_CENTERED,
            DisplayModeScaling::Stretched => DXGI_MODE_SCALING_STRETCHED
        }
    }
}

impl ScanlineOrder {
    pub fn to_dx(&self) -> DXGI_MODE_SCANLINE_ORDER {
        match self {
            ScanlineOrder::Unspecified => DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
            ScanlineOrder::Progressive => DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
            ScanlineOrder::UpperField => DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST,
            ScanlineOrder::LowerField => DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST,
        }
    }
}

impl SwapEffect {
    pub fn to_dx(&self) -> DXGI_SWAP_EFFECT {
        match self {
            SwapEffect::Discard => DXGI_SWAP_EFFECT_DISCARD,
            SwapEffect::Sequential => DXGI_SWAP_EFFECT_SEQUENTIAL,
            SwapEffect::FlipSequential => DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            SwapEffect::FlipDiscard => DXGI_SWAP_EFFECT_FLIP_DISCARD
        }
    }
}

impl SurfaceUsage{
    pub fn to_dx(&self) -> DXGI_USAGE {
        match self {
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

impl SwapChainFlag {
    pub fn to_dx(&self) -> DXGI_SWAP_CHAIN_FLAG {
        match self {
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