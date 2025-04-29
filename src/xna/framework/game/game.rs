use crate::xna::csharp::TimeSpan;
use crate::xna::framework::game::{Game, GameTime};

impl Game {
    pub fn new() -> Self {
        Game {
            game_window: None,
            graphics_device: None,
            game_time: GameTime::default(),
            is_fixed_time_step: true,
            target_elapsed_time: TimeSpan::from_ticks(166667)
        }
    }
}