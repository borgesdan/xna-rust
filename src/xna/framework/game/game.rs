use crate::xna::csharp::TimeSpan;
use crate::xna::framework::game::{Game, GameTime};

impl Game {
    pub fn new() -> Self {
        Game {
            is_fixed_time_step: true,
            target_elapsed_time: TimeSpan::from_ticks(166667),
                ..Default::default()
        }
    }
}