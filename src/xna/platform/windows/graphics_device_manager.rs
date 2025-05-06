use crate::xna::csharp::Exception;
use crate::xna::framework::AsBase;
use crate::xna::framework::game::{GraphicsDeviceInformation, GraphicsDeviceManager};
use crate::xna::framework::graphics::{GraphicsAdapter, PresentInterval, PresentationParameters};
use crate::xna::platform::windows::{WindowsGraphicsDeviceManager, WindowsPresentationParameters};

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
        let game_window = game.game_window.as_ref().unwrap();
        let screen_device_name = game_window.scree_device_name().unwrap();
        let client_bounds = game_window.client_bounds();
        let client_width = client_bounds.width;
        let client_height = client_bounds.height;
        let mut flag1 = false;

        //let best_device = self.find_best_device(force_create);

        //TODO
        Ok(())

    }

    // fn find_best_device(&mut self, force_create: bool) -> Option<&mut GraphicsDeviceManager>{
    //
    // }
    //
    // fn find_best_platform_device(&mut self, any_suitable_device: bool) -> Result<GraphicsDeviceInformation, Exception>{
    //     let found_devices : Vec<GraphicsDeviceInformation> = Vec::new();
    //
    //     add_devices(any_suitable_device, &found_devices);
    // }
    //
    // fn add_devices(&self, any_suitable_device: bool, found_devices: &Vec<GraphicsDeviceInformation>) {
    //     let handle = self.game.as_ref().unwrap()
    //         .game_window.as_ref().unwrap()
    //         .platform.hwnd;
    //
    //     let adapters = GraphicsAdapter::adapters();
    //
    //     for i in adapters {
    //         let base_device_info = GraphicsDeviceInformation {
    //             adapter: i,
    //             profile: self.graphics_profile,
    //             presentation_parameters: PresentationParameters {
    //                 platform: WindowsPresentationParameters {
    //                     hwnd: handle,
    //                 },
    //                 multi_sample_count: 0,
    //                 is_full_screen: self.is_full_screen,
    //                 presentation_interval: if self.synchronize_with_vertical_retrace { PresentInterval::Default} else { PresentInterval::Immediate},
    //                 ..Default::default()
    //             }
    //         };
    //
    //         //let current_display_mode = base_device_info.adapter.
    //     }
    // }
}