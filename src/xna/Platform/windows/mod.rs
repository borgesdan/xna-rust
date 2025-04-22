use windows::core::BOOL;
use windows::Win32::Foundation::{FALSE, TRUE};
use windows::Win32::Graphics::Direct3D11::{D3D11_BLEND, D3D11_BLEND_BLEND_FACTOR, D3D11_BLEND_DEST_ALPHA,
                                           D3D11_BLEND_DEST_COLOR, D3D11_BLEND_INV_BLEND_FACTOR,
                                           D3D11_BLEND_INV_DEST_ALPHA, D3D11_BLEND_INV_DEST_COLOR,
                                           D3D11_BLEND_INV_SRC1_ALPHA, D3D11_BLEND_INV_SRC1_COLOR,
                                           D3D11_BLEND_INV_SRC_ALPHA, D3D11_BLEND_INV_SRC_COLOR,
                                           D3D11_BLEND_ONE, D3D11_BLEND_OP, D3D11_BLEND_OP_ADD,
                                           D3D11_BLEND_OP_MAX, D3D11_BLEND_OP_MIN, D3D11_BLEND_OP_REV_SUBTRACT,
                                           D3D11_BLEND_OP_SUBTRACT, D3D11_BLEND_SRC1_ALPHA, D3D11_BLEND_SRC1_COLOR,
                                           D3D11_BLEND_SRC_ALPHA, D3D11_BLEND_SRC_ALPHA_SAT, D3D11_BLEND_SRC_COLOR,
                                           D3D11_BLEND_ZERO, D3D11_COLOR_WRITE_ENABLE, D3D11_COLOR_WRITE_ENABLE_ALL,
                                           D3D11_COLOR_WRITE_ENABLE_ALPHA, D3D11_COLOR_WRITE_ENABLE_BLUE,
                                           D3D11_COLOR_WRITE_ENABLE_GREEN, D3D11_COLOR_WRITE_ENABLE_RED};
use crate::xna::framework::graphics::{Blend, BlendFunction, ColorWriteChannels};

pub fn bool_to_win_bool(bool: bool) -> BOOL {
    if bool { TRUE } else { FALSE }
}

pub fn blend_to_d3dx_blend(blend: &Blend) -> D3D11_BLEND {
    match blend {
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
        _ => D3D11_BLEND_ZERO,
    }
}

pub fn blend_operation_to_d3dx_blend_op(op: &BlendFunction) -> D3D11_BLEND_OP {
    match op {
        BlendFunction::Add => D3D11_BLEND_OP_ADD,
        BlendFunction::Max => D3D11_BLEND_OP_MAX,
        BlendFunction::Min => D3D11_BLEND_OP_MIN,
        BlendFunction::ReverseSubtract => D3D11_BLEND_OP_REV_SUBTRACT,
        BlendFunction::Subtract => D3D11_BLEND_OP_SUBTRACT,
        _ => D3D11_BLEND_OP_ADD,
    }
}

pub fn color_write_channels_to_d3dx_color_write_enable(channel: &ColorWriteChannels) -> D3D11_COLOR_WRITE_ENABLE {
    match channel {
        ColorWriteChannels::Red => D3D11_COLOR_WRITE_ENABLE_RED,
        ColorWriteChannels::Green => D3D11_COLOR_WRITE_ENABLE_GREEN,
        ColorWriteChannels::Blue => D3D11_COLOR_WRITE_ENABLE_BLUE,
        ColorWriteChannels::Alpha => D3D11_COLOR_WRITE_ENABLE_ALPHA,
        ColorWriteChannels::All => D3D11_COLOR_WRITE_ENABLE_ALL,
        _ => D3D11_COLOR_WRITE_ENABLE_ALL,
    }
}