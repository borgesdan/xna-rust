use windows::Win32::Graphics::Direct3D11::D3D11_RASTERIZER_DESC;
use crate::xna::framework::graphics::RasterizerState;
use crate::xna::platform::windows::bool_to_win_bool;

impl RasterizerState {
    pub fn to_dx(&self) -> D3D11_RASTERIZER_DESC {
        D3D11_RASTERIZER_DESC {
            CullMode: self.cull_mode.to_dx(),
            AntialiasedLineEnable: bool_to_win_bool(self.antialiased_line_enable),
            ScissorEnable: bool_to_win_bool(self.scissor_test_enable),
            SlopeScaledDepthBias: self.slope_scale_depth_bias,
            DepthBias: self.depth_bias,
            MultisampleEnable: bool_to_win_bool(self.multi_sample_anti_alias),
            FillMode: self.fill_mode.to_dx(),
            DepthBiasClamp: self.depth_bias_clamp,
            DepthClipEnable: bool_to_win_bool(self.depth_clip_enable),
            FrontCounterClockwise: bool_to_win_bool(self.front_counter_clockwise),
        }
    }
}