use crate::xna::csharp::Exception;
use crate::xna::framework::game::{Game, GraphicsDeviceManager, GraphicsProfile};
use crate::xna::framework::graphics::{GraphicsAdapter, GraphicsDevice, PresentationParameters, SurfaceFormat};

impl GraphicsDeviceManager {
    pub const DEFAULT_BACK_BUFFER_WIDTH: u32 = 800;
    pub const DEFAULT_BACK_BUFFER_HEIGHT: u32 = 480;
}

impl GraphicsDeviceManager {
    pub fn new(game: Option<Box<Game>>) -> Self {
        let adapter = GraphicsAdapter::default();

        let parameters = PresentationParameters {
            back_buffer_width: Self::DEFAULT_BACK_BUFFER_WIDTH,
            back_buffer_height: Self::DEFAULT_BACK_BUFFER_HEIGHT,
            back_buffer_format: SurfaceFormat::Color,
            is_full_screen: false,
            ..Default::default()
        };

        GraphicsDeviceManager {
            game: game.clone(),
            graphics_adapter: Some(Box::new(adapter)),
            presentation_parameters: parameters,
            ..Default::default()
        }

    }
}

impl GraphicsDeviceManager {
    pub fn get_graphics_adapter(&self) -> Option<Box<GraphicsAdapter>> {
        self.graphics_adapter.clone()
    }

    pub fn get_graphics_device(&self) -> Option<Box<GraphicsDevice>> {
        self.graphics_device.clone()
    }

    pub fn get_game(&self) -> Option<Box<Game>> {
        self.game.clone()
    }
}