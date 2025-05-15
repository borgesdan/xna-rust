use crate::xna::csharp::TimeSpan;
use crate::xna::framework::game::{Game, GameWindow, GraphicsDeviceManager};
use std::cell::RefCell;
use std::rc::Rc;

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            is_fixed_time_step: true,
            target_elapsed_time: TimeSpan::from_ticks(166667),
            game_window: Some(Rc::new(RefCell::new(GameWindow::new(
                "My Game",
                GraphicsDeviceManager::DEFAULT_BACK_BUFFER_WIDTH,
                GraphicsDeviceManager::DEFAULT_BACK_BUFFER_HEIGHT)))),
            ..Default::default()
        };

        game.set_is_fixed_time_step(game.is_fixed_time_step.clone());
        game.set_target_elapsed_time(game.target_elapsed_time.clone());

        game
    }
}