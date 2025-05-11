use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::xna::csharp::{Exception, TimeSpan};
use crate::xna::framework::game::{Game, GameTime, GraphicsDeviceManager};
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, DispatchMessageW, GetMessageW, PeekMessageA, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_CLOSE, WM_DESTROY, WM_NCLBUTTONDOWN, WM_QUIT};
use crate::xna::framework::graphics::GraphicsDevice;
use crate::xna::platform::windows::StepTimer;
use crate::xna::UnboxRc;

impl Game {
    pub fn exit(&mut self) -> Result<(), Exception> {
        let mut gw = self.game_window.unbox()?;
        gw.borrow_mut().close()
    }

    fn start_game_loop(&mut self) -> Result<usize, Exception> {
        self.platform.step_timer = StepTimer::new();

        let mut msg = MSG::default();

        loop {
            let gw_temp = self.game_window.unbox()?;
            let game_window = gw_temp.borrow();

            unsafe{
                if GetMessageW(&mut msg, Some(game_window.platform.hwnd), 0, 0).into() {
                            let _ = TranslateMessage(&msg);
                            let _ = DispatchMessageW(&msg);

                            if msg.message == WM_QUIT || msg.message == 0 {
                                break;
                            }
                } else {
                    self.tick()?;
                }
            }
        }

        if self.end_run_fn.is_some() {
            self.end_run_fn.unwrap()()?;
        }

        Ok(msg.wParam.0)
    }

    fn tick(&mut self) -> Result<(), Exception> {
        let mut timer = self.platform.step_timer.clone();

        let mut lambda=  || -> Result<(), Exception> {
            let elapsed = self.platform.step_timer.get_elapsed_seconds();
            let total = self.platform.step_timer.get_total_seconds();
            let elapsed_time_span = TimeSpan::from_seconds(elapsed as i32)?;
            let total_time_span = TimeSpan::from_seconds(total as i32)?;
            self.current_game_time.elapsed_time = elapsed_time_span;
            self.current_game_time.total_time = total_time_span;

            let current_game_time = self.current_game_time.clone();
            self.update(&current_game_time)?;

            Ok(())
        };

        timer.tick(&mut lambda)?;

        self.platform.step_timer = timer;

        self.begin_draw()?;
        let current_game_time = self.current_game_time.clone();
        self.draw(&current_game_time)?;
        self.end_draw()?;

        Ok(())
    }

    pub fn create_window(&mut self) -> Result<(), Exception> {
        if self.platform.is_running {
            return Ok(());
        }

        let mut game_window = self.game_window.unbox()?;
        game_window.borrow_mut().create()?;

        self.is_window_created = true;

        Ok(())
    }

    pub fn run(&mut self) -> Result<usize, Exception> {
        if self.platform.is_running {
            return Ok(1)
        }

        if !self.is_window_created {
            return Err(Exception::new("Window is not running", None));
        }

        self.initialize()?;

        self.platform.is_running = true;
        self.begin_run()?;
        self.start_game_loop()?;

        Ok(0)
    }

    fn initialize(&mut self) -> Result<(), Exception> {
        if self.initialize_fn.is_some() {
            self.initialize_fn.as_ref().unwrap()()?;
        }

        self.load_content()
    }

    fn load_content(&mut self) -> Result<(), Exception> {
        if self.load_content_fn.is_some() {
            self.load_content_fn.as_ref().unwrap()()?;
        }

        Ok(())
    }

    fn update(&mut self, game_time: &GameTime) -> Result<(), Exception> {
        if self.update_fn.is_some() {
            self.update_fn.unwrap()(game_time)?
        }

        Ok(())
    }

    fn begin_draw(&self)-> Result<(), Exception> {
        if self.begin_fn.is_some() {
            self.begin_fn.unwrap()()?
        }

        Ok(())
    }

    fn draw(&mut self, game_time: &GameTime)-> Result<(), Exception> {
        if self.draw_fn.is_some() {
            self.draw_fn.unwrap()(game_time)?
        }

        if self.graphics_device.is_none() {
            return Ok(());
        }

        self.graphics_device
            .unbox()?
            .borrow()
            .present();

        Ok(())
    }

    fn end_draw(&mut self)-> Result<(), Exception> {
        if self.end_fn.is_some() {
            self.end_fn.unwrap()()?
        }

        Ok(())
    }

    fn begin_run(&mut self)-> Result<(), Exception> {
        if self.begin_run_fn.is_some() {
            self.begin_run_fn.unwrap()()?
        }

        Ok(())
    }

    pub fn resize_window(&mut self, width: u32, height: u32) -> Result<(), Exception> {
        let gw_temp = self.game_window.unbox()?;
        let mut game_window = gw_temp.borrow_mut();

        let windows_bounds = game_window.client_bounds();

        if windows_bounds.width != width as i32 || windows_bounds.height != height as i32 {
            game_window.width = width;
            game_window.height = height;

            game_window.update()?
        }

        Ok(())
    }

    pub fn attach_graphics_device(&mut self, device: Rc<RefCell<GraphicsDevice>>) {
        self.graphics_device = Some(device);
    }

    pub fn run_one_frame(&mut self) -> Result<(), Exception> {
        self.tick()
    }

    pub fn reset_elapsed_time(&mut self) -> Result<(), Exception> {
        self.platform.step_timer.reset_elapsed_time()
    }

    pub fn set_target_elapsed_time(&mut self, value: TimeSpan) {
        if !self.is_fixed_time_step{
            return;
        }

        self.platform.step_timer.set_target_elapsed_ticks(value.ticks as u64);
    }

    pub fn set_is_fixed_time_step(&mut self, value: bool) {
        self.is_fixed_time_step = value;
        self.platform.step_timer.set_fixed_time_step(value);
    }

}