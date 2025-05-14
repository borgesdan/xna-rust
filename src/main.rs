mod xna;

use std::cell::{RefCell, RefMut};
use std::error::Error;
use std::fmt::Pointer;
use std::rc::Rc;
use crate::xna::csharp::Exception;
use crate::xna::framework::Color;
use crate::xna::framework::game::{Game, GameHandler, GameTime, GraphicsDeviceManager};

fn main() {
    let mut game = Rc::new(RefCell::new(Game::new()));
    let mut graphics_device_manager = Rc::new(RefCell::new(GraphicsDeviceManager::new(Some(game.clone()))));

    let mut game1 = Rc::new(RefCell::new(Game1 {
        graphics_device_manager: graphics_device_manager.clone(),
        game: game.clone(),
    }));

    create_window(game.clone());
    apply_graphics_device_manager(graphics_device_manager.clone());

    let mut borrow = game.borrow_mut();
    borrow.handler = Some(game1.clone());
    let result = borrow.run();

    if result.is_err() {
        let error = result.err().unwrap();
        println!("{}", error.message);
        std::process::exit(1);
    }
}

fn create_window(game: Rc<RefCell<Game>>) {
    let mut borrow = game.borrow_mut();
    let result = borrow.create_window();

    if result.is_err() {
        let error = result.err().unwrap();
        println!("{}", error.message);
        std::process::exit(1);
    }
}

fn apply_graphics_device_manager(graphics_device_manager: Rc<RefCell<GraphicsDeviceManager>>) {
    let mut device_borrow = graphics_device_manager.borrow_mut();
    let result = device_borrow.apply_changes();

    if result.is_err() {
        let error = result.err().unwrap();
        println!("{}", error.message);
        std::process::exit(1);
    }
}

struct Game1 {
    pub graphics_device_manager: Rc<RefCell<GraphicsDeviceManager>>,
    pub game: Rc<RefCell<Game>>,
}

impl GameHandler for Game1 {
    fn on_begin_run(&mut self) -> Result<(), Exception> {
        Ok(())
    }

    fn on_end_run(&mut self) -> Result<(), Exception> {
        Ok(())
    }

    fn on_update(&mut self, game_time: &GameTime) -> Result<(), Exception> {
        Ok(())
    }

    fn on_draw(&mut self, game_time: &GameTime) -> Result<(), Exception> {
        let manager = self.graphics_device_manager.borrow_mut();
        let device = manager.graphics_device.as_ref().unwrap().borrow_mut();
        device.clear(Color::cornflower_blue())?;

        Ok(())
    }

    fn on_begin_draw(&mut self) -> Result<(), Exception> {
        Ok(())
    }

    fn on_end_draw(&mut self) -> Result<(), Exception> {
        Ok(())
    }

    fn on_initialize(&mut self) -> Result<(), Exception> {
        Ok(())
    }

    fn on_load_content(&mut self) -> Result<(), Exception> {
        Ok(())
    }
}

