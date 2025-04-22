use std::iter::Once;
use crate::xna::framework::Color;
use crate::xna::framework::graphics::{Blend, BlendFunction, BlendRenderTarget, BlendState, ColorWriteChannels};

impl BlendState {
    pub fn new() -> BlendState {
        BlendState {
            alpha_to_coverage_enable: false,
            independent_blend_enable: false,
            blend_factor: Color::white(),
            multi_sample_mask: 0xffffffff,
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
        let mut blend = BlendState::new();
        blend.render_targets[0].source = Blend::One;
        blend.render_targets[0].source_alpha = Blend::One;
        blend.render_targets[0].destination = Blend::Zero;
        blend.render_targets[0].destination_alpha = Blend::Zero;

        blend
    }

    pub fn alpha_blend() -> BlendState {
        let mut blend = BlendState::new();
        blend.render_targets[0].source = Blend::One;
        blend.render_targets[0].source_alpha = Blend::One;
        blend.render_targets[0].destination = Blend::InverseSourceAlpha;
        blend.render_targets[0].destination_alpha = Blend::InverseSourceAlpha;

        blend
    }

    pub fn additive() -> BlendState {
        let mut blend = BlendState::new();
        blend.render_targets[0].source = Blend::SourceAlpha;
        blend.render_targets[0].source_alpha = Blend::SourceAlpha;
        blend.render_targets[0].destination = Blend::One;
        blend.render_targets[0].destination_alpha = Blend::One;

        blend
    }

    pub fn non_premultiplied() -> BlendState {
        let mut blend = BlendState::new();
        blend.render_targets[0].source = Blend::SourceAlpha;
        blend.render_targets[0].source_alpha = Blend::SourceAlpha;
        blend.render_targets[0].destination = Blend::InverseSourceAlpha;
        blend.render_targets[0].destination_alpha = Blend::InverseSourceAlpha;

        blend
    }
}