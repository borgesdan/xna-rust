use crate::xna::csharp::Exception;
use crate::xna::framework::AsBase;
use crate::xna::framework::game::GraphicsDeviceManager;
use crate::xna::platform::windows::WindowsGraphicsDeviceManager;

impl GraphicsDeviceManager{
    pub fn apply_changes(&mut self) -> Result<(), Exception> {
        if self.graphics_device.is_some() && !self.device_dirty {
            return Ok(())
        }

        self.change_device(false)
    }

    pub fn change_device(&mut self, force_create: bool) -> Result<(), Exception> {
        if self.game.is_none() {
            return Err(Exception::new("Game is not defined.", None));
        }

        self.in_device_transition = true;
        let game = self.game.as_ref().unwrap();
        let screen_device_name = game.game_window.as_ref().unwrap().scree_device_name().unwrap();

        //TODO
        Ok(())

    }
}