impl crate::xna::framework::game::GameWindow {
    pub fn client_bounds(&self) -> crate::xna::framework::Rectangle {
        crate::xna::framework::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}