use crate::xna::framework::graphics::{DepthStencilState, DepthStencilStateStencil};

impl DepthStencilStateStencil {
    pub fn none() -> DepthStencilState {
        DepthStencilState {
            depth_buffer_enable: true,
            depth_buffer_write_enable: false,
            ..Default::default()
        }
    }

    pub fn default() -> DepthStencilState {
        DepthStencilState {
            depth_buffer_enable: true,
            depth_buffer_write_enable: true,
            ..Default::default()
        }
    }

    pub fn depth_read() -> DepthStencilState {
        DepthStencilState {
            depth_buffer_enable: true,
            depth_buffer_write_enable: true,
            ..Default::default()
        }
    }
}