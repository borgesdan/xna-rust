use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::xna::csharp::Exception;
use crate::xna::framework::game::{GameWindow, GraphicsDeviceInformation, GraphicsDeviceManager};
use crate::xna::framework::graphics::{DisplayMode, GraphicsAdapter, GraphicsDevice, PresentInterval, PresentationParameters};
use crate::xna::platform::windows::{WinErrorException, WindowsPresentationParameters};
use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::WindowsAndMessaging::GetClientRect;
use crate::xna::UnboxRc;

impl GraphicsDeviceManager{
    pub fn apply_changes(&mut self) -> Result<(), Exception> {
        if self.graphics_device.is_some() && !self.is_device_dirty {
            return Ok(())
        }

        self.change_device(false)
    }

    pub fn toggle_full_screen(&mut self) -> Result<(), Exception> {
        let temp_device = self.graphics_device.unbox()?;
        let mut device = temp_device.borrow_mut();
        let mut swap_chain = device.platform.swap_chain.as_mut().unwrap();

        let mut state = BOOL(0);
        unsafe {
            let mut result = swap_chain.GetFullscreenState(Some(&mut state), None);

            if result.is_err() {
                let error = result.err().unwrap().to_exception();
                return Err(Exception::new("Toggle full screen fail", Some(error)));
            }

            result = swap_chain.SetFullscreenState(!state.as_bool(), None);

            if result.is_err() {
                let error = result.err().unwrap().to_exception();
                return Err(Exception::new("Toggle full screen failt", Some(error)));
            }
        }

        self.is_full_screen = !state.as_bool();

        Ok(())
    }

    fn change_device(&mut self, force_create: bool) -> Result<(), Exception> {
        if self.game.is_none() {
            return Err(Exception::new("Game is not defined.", None));
        }

        self.in_device_transition = true;

        let mut best_device = self.find_best_platform_device(force_create)?;
        let mut flag = true;

        if !force_create && self.graphics_device.is_some() {
            let can_reset = self.can_reset_device(&best_device)?;

            if can_reset {
                let mut pp = best_device.presentation_parameters.clone();
                self.message_present_parameters(&mut pp)?;
                Self::validate_graphics_device_information(&best_device)?;

                let temp_device = self.graphics_device.unbox()?;
                let mut device = temp_device.borrow_mut();
                device.reset(&best_device.presentation_parameters, &best_device.adapter)?;

                flag = false;
            }
        }

        if flag {
            self.create_device(&best_device)?;
        }

        self.is_device_dirty = false;
        self.in_device_transition = false;

        Ok(())

    }

    fn find_best_platform_device(&mut self, any_suitable_device: bool) -> Result<GraphicsDeviceInformation, Exception>{

        let mut found_devices : Vec<GraphicsDeviceInformation> = Vec::new();

        self.add_devices(any_suitable_device, &mut found_devices)?;

        if found_devices.len() == 0 && self.allow_multi_sampling {
            self.prefer_multi_sampling(false);
            self.add_devices(any_suitable_device, &mut found_devices)?;
        }

        if found_devices.len() == 0 {
            return Err(Exception::new("No devices found.", None));
        }

        Self::rank_devices_platform(&mut found_devices);

        if found_devices.len() == 0 {
            return Err(Exception::new("No devices found.", None));
        }

        Ok(found_devices[0].clone())
    }

    fn rank_devices_platform(found_devices: &mut Vec<GraphicsDeviceInformation>) {
        found_devices.sort()
    }

    fn create_device(&mut self, graphics_device_information: &GraphicsDeviceInformation) -> Result<(), Exception> {
        self.graphics_device = None;
        let mut new_info = graphics_device_information.clone();

        self.message_present_parameters(&mut new_info.presentation_parameters)?;
        Self::validate_graphics_device_information(&mut new_info)?;

        let width = new_info.presentation_parameters.back_buffer_width;
        let height = new_info.presentation_parameters.back_buffer_height;

        let temp_game = self.game.unbox()?;
        let mut game = temp_game.borrow_mut();

        game.resize_window(width, height)?;

        let device = Rc::new(RefCell::new(GraphicsDevice::new_from_profile(&new_info.adapter, &new_info.profile, &new_info.presentation_parameters)));
        self.graphics_device = Some(device.clone());
        game.attach_graphics_device(device.clone());

        let mut temp_device = device.borrow_mut();
        temp_device.initialize()?;

        Ok(())
    }

    fn add_devices(&self, any_suitable_device: bool, found_devices: &mut Vec<GraphicsDeviceInformation>)
    -> Result<(), Exception> {
        let temp_game = self.game.unbox()?;
        let game = temp_game.borrow();
        let temp_game_window = game.game_window.unbox()?;
        let game_window = temp_game_window.borrow();

        let handle = game_window.platform.hwnd;

        let adapters = GraphicsAdapter::adapters()?;

        for adapter in adapters {

            if !any_suitable_device {
                let on_adapter = Self::is_window_on_adapter(&handle, &adapter)?;

                if !on_adapter {
                    continue;
                }
            }

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
        } else if self.use_resized_back_buffer {
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

    fn can_reset_device(&self, new_device_info: &GraphicsDeviceInformation) -> Result<bool, Exception> {
        let profile = self.graphics_device
            .unbox()?
            .borrow()
            .graphics_profile
            .clone();

        Ok(profile == new_device_info.profile)
    }

    fn message_present_parameters(&self, parameters: &mut PresentationParameters) -> Result<(), Exception> {
        if parameters.is_full_screen {
            return Ok(());
        }

        let flag1 = parameters.back_buffer_width == 0;
        let flag2 = parameters.back_buffer_height == 0;

        if parameters.platform.hwnd.is_invalid() {
            if self.game.is_none() {
                return Err(Exception::invalid_operation("Graphics component not attached to game", None));
            }

            let hwnd = self.game
                .unbox()?
                .borrow()
                .game_window
                .unbox()?
                .borrow()
                .platform.hwnd
                .clone();

            parameters.platform.hwnd = hwnd;
        }

        let mut rect = RECT::default();

        unsafe{
            let result = GetClientRect(parameters.platform.hwnd.clone(), &mut rect);

            if result.is_err() {
                let inner = Exception::new(result.err().unwrap().message().as_str(), None);

                return Err(Exception::invalid_operation("Graphics component not attached to game", Some(inner)));
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

    fn validate_graphics_device_information(device_info: &GraphicsDeviceInformation) -> Result<(), Exception> {
        let presentation_parameters = &device_info.presentation_parameters;

        if presentation_parameters.is_full_screen {
            return Ok(())
        }

        if presentation_parameters.back_buffer_height == 0 || presentation_parameters.back_buffer_width == 0 {
            return Err(Exception::argument_exception("Validate backbuffer full screen fail", None));
        }

        let mut flag = true;
        let adapter = &device_info.adapter;
        let current_output = adapter.current_output.as_ref().unwrap();
        let current_display_mode = adapter.current_output.as_ref().unwrap().current_display_mode.as_ref().unwrap();

        if current_display_mode.format != presentation_parameters.back_buffer_format
            && current_display_mode.width != presentation_parameters.back_buffer_width
            && current_display_mode.height != presentation_parameters.back_buffer_height {

            flag = false;

            for display_mode in &current_output.display_mode_collection.display_modes {
                if display_mode.format == presentation_parameters.back_buffer_format && display_mode.width == presentation_parameters.back_buffer_width && display_mode.height == presentation_parameters.back_buffer_height {
                    flag = true;
                    break;
                }
            }
        }

        if !flag{
            return Err(Exception::argument_exception("Validate backbuffer full screen fail", None));
        }

        Ok(())
    }

    fn is_window_on_adapter(handle: &HWND, adapter: &GraphicsAdapter) ->Result<bool, Exception> {
        let from_handle = GameWindow::screen_from_handle(handle)?;
        let from_adapter = GameWindow::screen_from_adapter(adapter)?;

        Ok(from_handle.is_some() && from_adapter.is_some() && from_handle == from_adapter)
    }
}
