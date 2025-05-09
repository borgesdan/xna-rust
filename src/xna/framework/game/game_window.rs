use std::ops::Deref;
use crate::xna::csharp::Exception;
use crate::xna::csharp::forms::Screen;
use crate::xna::framework::game::GameWindow;
use crate::xna::framework::graphics::GraphicsAdapter;

impl GameWindow {
    pub fn client_bounds(&self) -> crate::xna::framework::Rectangle {
        crate::xna::framework::Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn screen_from_adapter(adapter: &GraphicsAdapter)-> Result<Option<Screen>, Exception> {
        let screens = Screen::all_screens();

        for screen in &screens {
            let device_name = &screen.device_name;
            let adp_device_name = &adapter.current_output.as_ref().unwrap().device_name;

            if adp_device_name == device_name {
                return Ok(Some(screen.clone()));
            }
        }

        Ok(None)
    }
}