use windows::Win32::Graphics::Direct3D11::D3D11_RASTERIZER_DESC;
use crate::xna::framework::graphics::RasterizerState;
use crate::xna::platform::windows::WinBool;

impl RasterizerState {
    pub fn to_dx(&self) -> D3D11_RASTERIZER_DESC {
        D3D11_RASTERIZER_DESC {
            CullMode: self.cull_mode.to_dx(),
            AntialiasedLineEnable: self.antialiased_line_enable.to_win_bool(),
            ScissorEnable: self.scissor_test_enable.to_win_bool(),
            SlopeScaledDepthBias: self.slope_scale_depth_bias,
            DepthBias: self.depth_bias,
            MultisampleEnable: self.multi_sample_anti_alias.to_win_bool(),
            FillMode: self.fill_mode.to_dx(),
            DepthBiasClamp: self.depth_bias_clamp,
            DepthClipEnable: self.depth_clip_enable.to_win_bool(),
            FrontCounterClockwise: self.front_counter_clockwise.to_win_bool(),
        }
    }
}