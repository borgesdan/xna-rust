use crate::xna::csharp::Exception;
use crate::xna::csharp::forms::Screen;
use crate::xna::framework::graphics::GraphicsAdapter;
impl Screen{
    pub fn from_adapter(adapter: &GraphicsAdapter) -> Result<Option<Screen>, Exception> {
        let screens = Screen::all_screens();

        for screen in &screens {
            if adapter.current_output.is_none(){
                continue;
            }

            let device_name = &screen.device_name;
            let adp_device_name = &adapter.current_output.as_ref().unwrap().device_name;

            if adp_device_name == device_name {
                return Ok(Some(screen.clone()));
            }
        }

        Ok(None)
    }
}

