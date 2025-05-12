use windows::Win32::Graphics::Direct3D11::{D3D11_BLEND, D3D11_BLEND_DESC, D3D11_BLEND_OP, D3D11_COLOR_WRITE_ENABLE};
use crate::xna::framework::graphics::BlendState;
use crate::xna::platform::windows::{WinBool};

impl BlendState{
    pub fn to_dx(&self) -> D3D11_BLEND_DESC {
        let mut description = D3D11_BLEND_DESC::default();
        description.AlphaToCoverageEnable = self.alpha_to_coverage_enable.to_win_bool();
        description.IndependentBlendEnable = self.independent_blend_enable.to_win_bool();

        let mut index = 0;
        for target in &self.render_targets {
            description.RenderTarget[index].BlendEnable = target.enabled.to_win_bool();
            description.RenderTarget[index].SrcBlend = D3D11_BLEND::from(target.source);
            description.RenderTarget[index].DestBlend = D3D11_BLEND::from(target.destination);
            description.RenderTarget[index].BlendOp = D3D11_BLEND_OP::from(target.operation);
            description.RenderTarget[index].SrcBlendAlpha = D3D11_BLEND::from(target.source_alpha);
            description.RenderTarget[index].DestBlendAlpha = D3D11_BLEND::from(target.destination_alpha);
            description.RenderTarget[index].BlendOpAlpha = D3D11_BLEND_OP::from(target.operation_alpha);
            description.RenderTarget[index].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE::from(target.write_mask).0 as u8;

            index += 1;
        }

        description
    }
}