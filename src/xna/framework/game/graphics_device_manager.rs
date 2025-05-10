use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::xna::csharp::Exception;
use crate::xna::framework::game::{Game, GraphicsDeviceManager, GraphicsProfile};
use crate::xna::framework::graphics::{DepthFormat, GraphicsAdapter, GraphicsDevice, PresentationParameters, SurfaceFormat};

impl GraphicsDeviceManager {
    pub const DEFAULT_BACK_BUFFER_WIDTH: u32 = 800;
    pub const DEFAULT_BACK_BUFFER_HEIGHT: u32 = 480;
}

impl GraphicsDeviceManager {
    pub fn new(game: Option<Rc<RefCell<Game>>>) -> Self {
        let adapter = GraphicsAdapter::default();

        let parameters = PresentationParameters {
            back_buffer_width: Self::DEFAULT_BACK_BUFFER_WIDTH,
            back_buffer_height: Self::DEFAULT_BACK_BUFFER_HEIGHT,
            back_buffer_format: SurfaceFormat::Color,
            is_full_screen: false,
            ..Default::default()
        };

        GraphicsDeviceManager {
            game: game.clone(),
            is_device_dirty: false,
            graphics_device: None,
            is_full_screen: false,
            graphics_profile: GraphicsProfile::HiDef,
            depth_stencil_format: DepthFormat::Depth24,
            back_buffer_format: SurfaceFormat::Color,
            synchronize_with_vertical_retrace: true,
            use_resized_back_buffer: false,
            allow_multi_sampling: false,
            in_device_transition: false,
            graphics_adapter: Some(Rc::new(RefCell::new(adapter))),
            presentation_parameters: parameters,
            ..Default::default()
        }

    }

    pub fn set_graphics_profile(&mut self, value: GraphicsProfile) {
        self.graphics_profile = value;
        self.is_device_dirty = true;
    }

    pub fn preferred_depth_stencil_format(&mut self, value: DepthFormat) {
        self.depth_stencil_format = value;
        self.is_device_dirty = true;
    }

    pub fn preferred_back_buffer_format(&mut self, value: SurfaceFormat) {
        self.back_buffer_format = value;
        self.is_device_dirty = true;
    }

    pub fn preferred_back_buffer_width(&mut self, value: u32) {
        self.resized_back_buffer_width = value;
        self.is_device_dirty = true;
    }

    pub fn preferred_back_buffer_height(&mut self, value: u32) {
        self.resized_back_buffer_height = value;
        self.is_device_dirty = true;
    }

    pub fn set_full_screen(&mut self, value: bool) {
        self.is_full_screen = value;
        self.is_device_dirty = true;
    }

    pub fn set_synchronize_with_vertical_retrace(&mut self, value: bool) {
        self.synchronize_with_vertical_retrace = value;
        self.is_device_dirty = true;
    }

    pub fn prefer_multi_sampling(&mut self,value: bool) {
        self.allow_multi_sampling = value;
        self.is_device_dirty = true;
    }
}