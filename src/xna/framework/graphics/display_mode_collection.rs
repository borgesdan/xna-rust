use crate::xna::framework::graphics::{DisplayMode, DisplayModeCollection, SurfaceFormat};

impl DisplayModeCollection {
    pub fn query(&self, format: &SurfaceFormat) -> DisplayModeCollection {
        let mut count :usize = 0;
        let mut modes: Vec<DisplayMode> = Vec::with_capacity(count);

        for mode in &self.display_modes {
            if mode.format == *format {
                modes.push(mode.clone());
                count += 1;
            }
        }

        if modes.len() != count {
            modes.resize(count, DisplayMode::default());
        }

        DisplayModeCollection {
            display_modes: modes,
        }
    }
}