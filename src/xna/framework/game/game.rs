use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::xna::csharp::TimeSpan;
use crate::xna::framework::game::{Game, GameWindow, GraphicsDeviceManager};

impl Game {
    pub fn new() -> Self {
        Game {
            is_fixed_time_step: true,
            target_elapsed_time: TimeSpan::from_ticks(166667),
            game_window: Some(Rc::new(RefCell::new(GameWindow::new(
                "My Game",
                GraphicsDeviceManager::DEFAULT_BACK_BUFFER_WIDTH,
                GraphicsDeviceManager::DEFAULT_BACK_BUFFER_HEIGHT)))),
            ..Default::default()
        }
    }
}