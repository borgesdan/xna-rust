use windows::Win32::Foundation::RECT;
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use crate::xna::csharp::Exception;
use crate::xna::framework::AsBase;
use crate::xna::framework::game::{GraphicsDeviceInformation, GraphicsDeviceManager};
use crate::xna::framework::graphics::{DepthFormat, DisplayMode, GraphicsAdapter, PresentInterval, PresentationParameters, SurfaceFormat};
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

        let best_device = self.find_best_device(force_create);

        if !force_create && self.graphics_device.is_some() {
            let can_reset = self.can_reset_device(&best_device);

            if can_reset {
                let device_information = &best_device;
                self.message_present_parameters(&best_device.parameters);
            }
        }

        //TODO
        Ok(())

    }

    pub fn prefer_multi_sampling(&mut self,value: bool) {
        self.allow_multi_sampling = value;
        self.is_device_dirty = true;
    }

    fn find_best_platform_device(&mut self) -> Result<GraphicsDeviceInformation, Exception>{

        let mut found_devices : Vec<GraphicsDeviceInformation> = Vec::new();

        self.add_devices(&mut found_devices)?;

        if found_devices.len() == 0 && self.allow_multi_sampling {
            self.prefer_multi_sampling(false);
            self.add_devices(&mut found_devices)?;
        }

        if found_devices.len() == 0 {
            return Err(Exception::new("No devices found.", None));
        }

        self.rank_devices(&mut found_devices);

        if found_devices.len() == 0 {
            return Err(Exception::new("No devices found.", None));
        }

        Self::rank_devices_platform(&mut found_devices);

        Ok(found_devices[0].clone())
    }

    fn rank_devices_platform(found_devices: &mut Vec<GraphicsDeviceInformation>) {
        found_devices.sort()
    }

    fn add_devices(&self, found_devices: &mut Vec<GraphicsDeviceInformation>)
    -> Result<(), Exception> {
        let handle = self.game.as_ref().unwrap()
            .game_window.as_ref().unwrap()
            .platform.hwnd;

        let adapters = GraphicsAdapter::adapters().unwrap();

        for adapter in adapters {
            let base_device_info = GraphicsDeviceInformation {
                adapter: adapter.clone(),
                profile: self.graphics_profile,
                presentation_parameters: PresentationParameters {
                    platform: WindowsPresentationParameters {
                        hwnd: handle,
                    },
                    multi_sample_count: 0,
                    is_full_screen: self.is_full_screen,
                    presentation_interval: if self.synchronize_with_vertical_retrace { PresentInterval::Default} else { PresentInterval::Immediate},
                    ..Default::default()
                }
            };

            let current_display_mode = base_device_info.adapter.current_output.as_ref().unwrap().current_display_mode.as_ref().unwrap();
            self.add_devices_with_display_mode(current_display_mode, &base_device_info, found_devices)?;

            if self.is_full_screen {
                let supported_display_modes = &adapter.current_output
                    .as_ref()
                    .unwrap().
                    display_mode_collection;

                for supported_mode in &supported_display_modes.display_modes {
                    if supported_mode.width >= 640 && supported_mode.height >= 480 {
                        self.add_devices_with_display_mode(&supported_mode, &base_device_info, found_devices)?
                    }
                }
            }
        }

        Ok(())
    }

    fn add_devices_with_display_mode(&self, mode: &DisplayMode,
                                     base_device_info: &GraphicsDeviceInformation,
                                     found_devices: &mut Vec<GraphicsDeviceInformation>)
        -> Result<(), Exception> {

        let mut device_information = base_device_info.clone();

        if self.is_full_screen {
            device_information.presentation_parameters.back_buffer_width = mode.width;
            device_information.presentation_parameters.back_buffer_height = mode.height;
        } else if self.use_resized_backbuffer {
            device_information.presentation_parameters.back_buffer_width = self.resized_back_buffer_width;
            device_information.presentation_parameters.back_buffer_height = self.resized_back_buffer_height;
        } else {
            device_information.presentation_parameters.back_buffer_width = self.resized_back_buffer_width;
            device_information.presentation_parameters.back_buffer_height = self.resized_back_buffer_height;
        }

        let query = base_device_info.adapter.query_back_buffer_format(
            &mode.format,
            &self.depth_stencil_format,
            if self.allow_multi_sampling {16} else {0}).unwrap();

        device_information.presentation_parameters.back_buffer_format = query.0;
        device_information.presentation_parameters.depth_stencil_format = query.1;
        device_information.presentation_parameters.multi_sample_count = query.2;

        if !found_devices.contains(&device_information) {
            found_devices.push(device_information);
        }

        Ok(())
    }

    fn can_reset_device(&self, new_device_info: &GraphicsDeviceInformation) -> bool{
        self.graphics_device.as_ref().unwrap().graphics_profile == new_device_info.profile
    }

    fn message_present_parameters(&self, parameters: &mut PresentationParameters) -> Result<(), Exception> {
        if parameters.is_full_screen {
            return;
        }

        let flag1 = parameters.back_buffer_width == 0;
        let flag2 = parameters.back_buffer_height == 0;

        if parameters.platform.hwnd.is_invalid() {
            if self.game.is_none() {
                return Err(Exception::invalid_operation("Graphics component not attached to game", None));
            }

            parameters.platform.hwnd = self.game.as_ref().unwrap()
                .game_window.as_ref().unwrap()
                .platform.hwnd.clone();
        }

        let mut rect = RECT::default();

        unsafe{
            let result = GetClientRect(parameters.platform.hwnd.clone(), &mut rect);

            if result.is_err() {
                let inner = Exception::new(result.err().unwrap().message().as_str(), None);

                return Err(Exception::invalid_operation("Graphics component not attached to game", Some(Box::new(inner))));
            }
        }

        if flag1 && rect.right == 0 {
            parameters.back_buffer_width = 1;
        }
        if flag2 || rect.bottom != 0 {
            return Ok(())
        }

        parameters.back_buffer_height = 1;

        Ok(())
    }
}