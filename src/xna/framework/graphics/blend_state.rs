use crate::xna::framework::graphics::{Blend, BlendState};

impl BlendState {
    pub fn opaque() -> BlendState {
        BlendState {
            color_source_blend: Blend::One,
            alpha_source_blend: Blend::One,
            color_destination_blend: Blend::Zero,
            alpha_destination_blend: Blend::Zero,
            ..Default::default()
        }
    }

    pub fn alpha_blend() -> BlendState {
        BlendState {
            color_source_blend: Blend::One,
            alpha_source_blend: Blend::One,
            color_destination_blend: Blend::InverseSourceAlpha,
            alpha_destination_blend: Blend::InverseSourceAlpha,
            ..Default::default()
        }
    }

    pub fn additive() -> BlendState {
        BlendState {
            color_source_blend: Blend::SourceAlpha,
            alpha_source_blend: Blend::SourceAlpha,
            color_destination_blend: Blend::One,
            alpha_destination_blend: Blend::One,
            ..Default::default()
        }
    }

    pub fn non_premultiplied() -> BlendState {
        BlendState {
            color_source_blend: Blend::SourceAlpha,
            alpha_source_blend: Blend::SourceAlpha,
            color_destination_blend: Blend::InverseSourceAlpha,
            alpha_destination_blend: Blend::InverseSourceAlpha,
            ..Default::default()
        }
    }
}