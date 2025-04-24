use crate::xna::framework::graphics::{DisplayMode, DisplayModeScaling, PresentationParameters, ScanlineOrder, SurfaceFormat, SurfaceUsage, SwapChain, SwapChainFlag, SwapEffect};

impl SwapChain {
    pub fn new() -> Self {
        SwapChain {
            windowed: true,
            sample_count: 1,
            sample_quality: 0,
            buffer_count: 2,
            usage: SurfaceUsage::RenderTargetOutput,
            flags: SwapChainFlag::AllowModeSwitch,
            display: DisplayMode{
                width: 800,
                height: 600,
                format: SurfaceFormat::Color,
                refresh_rate_numerator: 60,
                refresh_rate_denominator: 1,
                scaling: DisplayModeScaling::Unspecified,
                scanline_order: ScanlineOrder::Unspecified
            },
            swap_effect: SwapEffect::FlipDiscard,
        }
    }

    pub fn from_parameters(parameters: &PresentationParameters) -> Self {
        SwapChain {
            windowed: !parameters.is_full_screen,
            sample_count: 1,
            sample_quality: 0,
            buffer_count: 2,
            usage: SurfaceUsage::RenderTargetOutput,
            flags: SwapChainFlag::AllowModeSwitch,
            display: DisplayMode{
                width: parameters.back_buffer_width,
                height: parameters.back_buffer_height,
                format: parameters.back_buffer_format,
                refresh_rate_numerator: 60,
                refresh_rate_denominator: 1,
                scaling: DisplayModeScaling::Unspecified,
                scanline_order: ScanlineOrder::Unspecified
            },
            swap_effect: SwapEffect::FlipDiscard,
        }
    }
}