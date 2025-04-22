use crate::xna::framework::graphics::{ComparisonFunction, DepthFace, DepthStencilState, StencilOperation};

impl DepthStencilState {
    pub fn new() -> Self {
        DepthStencilState {
            depth_enable: true,
            stencil_enable: true,
            depth_function: ComparisonFunction::LessEquals,
            front_face: DepthFace {
                stencil_function: ComparisonFunction::Always,
                stencil_pass_operation: StencilOperation::Keep,
                stencil_fail_operation: StencilOperation::Keep,
                stencil_depth_fail_operation: StencilOperation::Keep,
            },
            back_face: DepthFace {
                stencil_function: ComparisonFunction::Always,
                stencil_pass_operation: StencilOperation::Keep,
                stencil_fail_operation: StencilOperation::Keep,
                stencil_depth_fail_operation: StencilOperation::Keep,
            },
            stencil_read_mask: u8::MAX,
            stencil_write_mask: u8::MAX,
            depth_write_mask: true,
        }

    }

    pub fn none() -> DepthStencilState {
        let mut state = Self::new();
        state.depth_enable = false;
        state.depth_write_mask = false;

        state
    }

    pub fn default() -> DepthStencilState {
        let mut state = Self::new();
        state
    }

    pub fn depth_read() -> DepthStencilState {
        Self::default()
    }
}