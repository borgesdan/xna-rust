use crate::xna::csharp::TimeSpan;
use crate::xna::framework::game::{Game, GameTime, GameWindow, GraphicsDeviceManager};

impl Game {
    pub fn new() -> Self {
        Game {
            is_fixed_time_step: true,
            target_elapsed_time: TimeSpan::from_ticks(166667),
            game_window: Some(GameWindow::new(
                "My Game",
                GraphicsDeviceManager::DEFAULT_BACK_BUFFER_WIDTH,
                GraphicsDeviceManager::DEFAULT_BACK_BUFFER_HEIGHT)),
                ..Default::default()
        }
    }
}