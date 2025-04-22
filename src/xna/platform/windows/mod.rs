pub mod depth_stencil_state;

use windows::core::BOOL;
use windows::Win32::Foundation::{FALSE, TRUE};
use windows::Win32::Graphics::Direct3D11::{D3D11_BLEND, D3D11_BLEND_BLEND_FACTOR, D3D11_BLEND_DEST_ALPHA, D3D11_BLEND_DEST_COLOR, D3D11_BLEND_INV_BLEND_FACTOR, D3D11_BLEND_INV_DEST_ALPHA, D3D11_BLEND_INV_DEST_COLOR, D3D11_BLEND_INV_SRC1_ALPHA, D3D11_BLEND_INV_SRC1_COLOR, D3D11_BLEND_INV_SRC_ALPHA, D3D11_BLEND_INV_SRC_COLOR, D3D11_BLEND_ONE, D3D11_BLEND_OP, D3D11_BLEND_OP_ADD, D3D11_BLEND_OP_MAX, D3D11_BLEND_OP_MIN, D3D11_BLEND_OP_REV_SUBTRACT, D3D11_BLEND_OP_SUBTRACT, D3D11_BLEND_SRC1_ALPHA, D3D11_BLEND_SRC1_COLOR, D3D11_BLEND_SRC_ALPHA, D3D11_BLEND_SRC_ALPHA_SAT, D3D11_BLEND_SRC_COLOR, D3D11_BLEND_ZERO, D3D11_COLOR_WRITE_ENABLE, D3D11_COLOR_WRITE_ENABLE_ALL, D3D11_COLOR_WRITE_ENABLE_ALPHA, D3D11_COLOR_WRITE_ENABLE_BLUE, D3D11_COLOR_WRITE_ENABLE_GREEN, D3D11_COLOR_WRITE_ENABLE_RED, D3D11_COMPARISON_ALWAYS, D3D11_COMPARISON_EQUAL, D3D11_COMPARISON_FUNC, D3D11_COMPARISON_GREATER, D3D11_COMPARISON_GREATER_EQUAL, D3D11_COMPARISON_LESS, D3D11_COMPARISON_LESS_EQUAL, D3D11_COMPARISON_NEVER, D3D11_COMPARISON_NOT_EQUAL, D3D11_STENCIL_OP, D3D11_STENCIL_OP_DECR, D3D11_STENCIL_OP_DECR_SAT, D3D11_STENCIL_OP_INCR, D3D11_STENCIL_OP_INCR_SAT, D3D11_STENCIL_OP_INVERT, D3D11_STENCIL_OP_KEEP, D3D11_STENCIL_OP_REPLACE, D3D11_STENCIL_OP_ZERO};
use crate::xna::framework::graphics::{Blend, BlendFunction, ColorWriteChannels, ComparisonFunction, StencilOperation};

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