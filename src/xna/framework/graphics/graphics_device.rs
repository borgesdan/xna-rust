use crate::xna::framework::game::GraphicsProfile;
use crate::xna::framework::graphics::{BlendState, DepthFormat, DepthStencilState, GraphicsAdapter, GraphicsDevice, PresentInterval, PresentationParameters, RasterizerState, RenderTarget2D, SurfaceFormat, SwapChain, SwapEffect, Texture2D, Viewport};

impl GraphicsDevice {
    pub fn new() -> Self {
        GraphicsDevice {
            presentation_parameters: PresentationParameters {
                presentation_swap_effect: SwapEffect::FlipDiscard,
                is_full_screen: false,
                back_buffer_width: 800,
                back_buffer_height:600,
                back_buffer_format: SurfaceFormat::Color,
                presentation_interval: PresentInterval::Default,
                depth_stencil_format: DepthFormat::None,
                multi_sample_count: 1,
                    ..Default::default()
            },
            render_target: RenderTarget2D {
                texture: Texture2D::default(),
                ..Default::default()
            },
            swap_chain: SwapChain::new(),
            blend_state: BlendState::opaque(),
            rasterizer_state: RasterizerState::cull_clock_wise(),
            depth_stencil_state: DepthStencilState::default(),
            viewport: Viewport {
                x: 0.0,
                y: 0.0,
                height: 600.0,
                width: 800.0,
                max_depth: 1.0,
                min_depth: 0.0
            },
            ..Default::default()
        }
    }

    pub fn new_from_profile(adapter: &GraphicsAdapter, profile: &GraphicsProfile, presentation_parameters: &PresentationParameters) -> Self {
        let mut device = Self::new();
        device.adapter = Some(adapter.clone());
        device.graphics_profile = profile.clone();
        device.presentation_parameters = presentation_parameters.clone();

        device
    }
}

