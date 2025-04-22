use windows::Win32::Graphics::Direct3D11::{D3D11_DEPTH_STENCILOP_DESC, D3D11_DEPTH_STENCIL_DESC, D3D11_DEPTH_WRITE_MASK};
use crate::xna::framework::graphics::{DepthFace, DepthStencilState};
use crate::xna::platform::windows::bool_to_win_bool;

impl DepthFace {
    pub fn to_dx(&self) -> D3D11_DEPTH_STENCILOP_DESC {
        D3D11_DEPTH_STENCILOP_DESC {
            StencilFunc: self.stencil_function.to_dx(),
            StencilPassOp: self.stencil_pass_operation.to_dx(),
            StencilDepthFailOp: self.stencil_depth_fail_operation.to_dx(),
            StencilFailOp: self.stencil_fail_operation.to_dx(),
        }
    }
}

impl DepthStencilState {
    pub fn to_desc(&self) -> D3D11_DEPTH_STENCIL_DESC {
        D3D11_DEPTH_STENCIL_DESC {
            DepthEnable: bool_to_win_bool(self.depth_enable),
            StencilEnable: bool_to_win_bool(self.stencil_enable),
            DepthFunc: self.depth_function.to_dx(),
            FrontFace: self.front_face.to_dx(),
            BackFace: self.back_face.to_dx(),
            StencilReadMask: self.stencil_read_mask,
            StencilWriteMask: self.stencil_write_mask,
            DepthWriteMask: D3D11_DEPTH_WRITE_MASK(self.depth_write_mask as i32),
        }
    }
}