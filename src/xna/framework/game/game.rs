use crate::xna::csharp::{Exception, TimeSpan};
use crate::xna::framework::game::{Game, GameTime, GameWindow, GraphicsDeviceManager};
use crate::xna::framework::graphics::GraphicsDevice;

impl Game {
    pub fn new() -> Self {
        Game {
            is_fixed_time_step: true,
            target_elapsed_time: TimeSpan::from_ticks(166667),
            game_window: Some(Box::new(GameWindow::new(
                "My Game",
                GraphicsDeviceManager::DEFAULT_BACK_BUFFER_WIDTH,
                GraphicsDeviceManager::DEFAULT_BACK_BUFFER_HEIGHT))),
            ..Default::default()
        }
    }

    pub fn get_game_window(&mut self) -> Result<&mut Box<GameWindow>, Exception> {
        if self.game_window.is_none() {
            return Err(Exception::invalid_operation("Game window is null", None));
        }

        Ok(self.game_window.as_mut().unwrap())
    }

    pub fn get_graphics_device(&mut self) -> Result<&mut Box<GraphicsDevice>, Exception> {
        if self.game_window.is_none() {
            return Err(Exception::invalid_operation("Graphics Device is null", None));
        }

        let device = self.get_graphics_device()?;

        Ok(device)
    }
}