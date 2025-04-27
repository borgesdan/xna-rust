use crate::xna::framework::graphics::{CullMode, FillMode, RasterizerState};

impl RasterizerState {
    pub fn new() -> Self {
        RasterizerState {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::None,
            depth_clip_enable: true,
            ..Default::default()
        }
    }

    pub fn cull_none() -> RasterizerState {
        RasterizerState {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::None,
            depth_clip_enable: true,
            ..Default::default()
        }
    }

    pub fn cull_clock_wise() -> RasterizerState {
        RasterizerState {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::CullClockwiseFace,
            depth_clip_enable: true,
            ..Default::default()
        }
    }

    pub fn cull_counter_clock_wise() -> RasterizerState {
        RasterizerState {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::CullCounterClockwiseFace,
            depth_clip_enable: true,
            ..Default::default()
        }
    }
}