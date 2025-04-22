use windows::Win32::Graphics::Direct3D11::D3D11_BLEND_DESC;
use crate::xna::framework::graphics::BlendState;
use crate::xna::platform::windows::bool_to_win_bool;

impl BlendState{
    pub fn to_dx(&self) -> D3D11_BLEND_DESC {
        let mut description = D3D11_BLEND_DESC::default();
        description.AlphaToCoverageEnable = bool_to_win_bool(self.alpha_to_coverage_enable);
        description.IndependentBlendEnable = bool_to_win_bool(self.independent_blend_enable);

        let mut index = 0;
        for target in &self.render_targets {
            description.RenderTarget[index].BlendEnable = bool_to_win_bool(target.enabled);
            description.RenderTarget[index].SrcBlend = target.source.to_dx();
            description.RenderTarget[index].DestBlend = target.destination.to_dx();
            description.RenderTarget[index].BlendOp = target.operation.to_dx();
            description.RenderTarget[index].SrcBlendAlpha = target.source_alpha.to_dx();
            description.RenderTarget[index].DestBlendAlpha = target.destination_alpha.to_dx();
            description.RenderTarget[index].BlendOpAlpha = target.operation_alpha.to_dx();
            description.RenderTarget[index].RenderTargetWriteMask = target.write_mask.to_dx().0 as u8;

            index += 1;
        }

        description
    }
}