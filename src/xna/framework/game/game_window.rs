use crate::xna::framework::game::GameWindow;

impl GameWindow {
    pub fn new(title: &str, width: u32, height: u32) -> GameWindow {
        GameWindow {
            width,
            height,
            title: title.to_string(),
            ..Default::default()
        }
    }

    pub fn client_bounds(&self) -> crate::xna::framework::Rectangle {
        crate::xna::framework::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width as i32,
            height: self.height as i32,
        }
    }
}