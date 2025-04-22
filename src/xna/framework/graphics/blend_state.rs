use std::iter::Once;
use crate::xna::framework::graphics::{Blend, BlendFunction, BlendRenderTarget, BlendState, ColorWriteChannels};

impl BlendState {
    pub fn new() -> BlendState {
        BlendState {
            alpha_to_coverage_enable: false,
            independent_blend_enable: false,
            render_targets: [BlendRenderTarget{
                enabled: true,
                source: Blend::One,
                destination: Blend::One,
                operation: BlendFunction::Add,
                source_alpha: Blend::One,
                destination_alpha: Blend::One,
                operation_alpha: BlendFunction::Add,
                write_mask: ColorWriteChannels::All,
            };8],
            ..Default::default()
        }
    }

    pub fn opaque() -> BlendState {
        BlendState {
            color_source_blend: Blend::One,
            alpha_source_blend: Blend::One,
            color_destination_blend: Blend::Zero,
            alpha_destination_blend: Blend::Zero,
            ..Self::new()
        }
    }

    pub fn alpha_blend() -> BlendState {
        BlendState {
            color_source_blend: Blend::One,
            alpha_source_blend: Blend::One,
            color_destination_blend: Blend::InverseSourceAlpha,
            alpha_destination_blend: Blend::InverseSourceAlpha,
            ..Self::new()
        }
    }

    pub fn additive() -> BlendState {
        BlendState {
            color_source_blend: Blend::SourceAlpha,
            alpha_source_blend: Blend::SourceAlpha,
            color_destination_blend: Blend::One,
            alpha_destination_blend: Blend::One,
            ..Self::new()
        }
    }

    pub fn non_premultiplied() -> BlendState {
        BlendState {
            color_source_blend: Blend::SourceAlpha,
            alpha_source_blend: Blend::SourceAlpha,
            color_destination_blend: Blend::InverseSourceAlpha,
            alpha_destination_blend: Blend::InverseSourceAlpha,
            ..Self::new()
        }
    }
}