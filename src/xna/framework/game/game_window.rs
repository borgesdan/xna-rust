impl crate::xna::framework::game::GameWindow {
    pub fn client_bounds(&self) -> crate::xna::framework::Rectangle {
        crate::xna::framework::Rectangle {
            x: self.window_pos_x,
            y: self.window_pos_y,
            width: self.window_width as i32,
            height: self.window_height as i32,
        }
    }
}