use crate::xna::csharp::{Exception, TimeSpan};
use crate::xna::framework::game::{Game, GameTime};
use crate::xna::framework::graphics::GraphicsDevice;
use crate::xna::platform::windows::StepTimer;
use crate::xna::SilentExceptionConverter;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, PeekMessageW, TranslateMessage, MSG, PM_REMOVE, WM_QUIT};

impl Game {
    pub fn exit(&mut self) -> Result<(), Exception> {
        let mut gw = self.game_window.unwrap_ref_or_default_exception()?;
        gw.borrow_mut().close()
    }

    fn start_game_loop(&mut self) -> Result<(), Exception> {
        self.platform.step_timer = StepTimer::new();

        let mut msg = MSG::default();
        let gw_temp = self.game_window.unwrap_ref_or_default_exception()?.clone();
        let game_window = gw_temp.borrow();

        loop {
            unsafe {
                // if GetMessageW(&mut msg, Some(game_window.platform.hwnd), 0, 0).as_bool(){
                //     let _ = TranslateMessage(&msg);
                //     let _ = DispatchMessageW(&msg);
                //
                //     //TODO: por algum motivo WM_QUIT não é enviado após a janela ser fechada.
                //     if msg.message == WM_QUIT || msg.message == 0 {
                //         break;
                //     }
                // } else {
                //     self.tick()?;
                // }

                if PeekMessageW(&mut msg, Some(game_window.platform.hwnd), 0, 0, PM_REMOVE).as_bool() {
                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageW(&msg);
                } else {
                    self.tick()?
                }

                if msg.message == WM_QUIT || msg.message == 0 {
                    break;
                }
            }
        }

        Ok(())
    }

    fn tick(&mut self) -> Result<(), Exception> {
        let mut timer = self.platform.step_timer.clone();

        let mut lambda = || -> Result<(), Exception> {
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

        let mut game_window = self.game_window.unwrap_ref_or_default_exception()?;
        game_window.borrow_mut().create()?;

        self.is_window_created = true;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        if self.platform.is_running {
            return Ok(());
        }

        if !self.is_window_created {
            return Err(Exception::new("Window is not running", None));
        }

        self.initialize()?;

        self.platform.is_running = true;

        if self.handler.is_some() {
            self.handler.as_ref().unwrap().borrow_mut().on_begin_run()?;
        }

        self.start_game_loop()?;

        if self.handler.is_some() {
            self.handler.as_ref().unwrap().borrow_mut().on_end_run()?;
        }

        Ok(())
    }

    fn initialize(&mut self) -> Result<(), Exception> {
        if self.handler.is_some() {
            self.handler.clone().as_mut().unwrap().borrow_mut().on_initialize()?;
        }

        self.load_content()
    }

    fn load_content(&mut self) -> Result<(), Exception> {
        if self.handler.is_some() {
            self.handler.clone().unwrap().borrow_mut().on_load_content()?;
        }

        Ok(())
    }

    fn update(&mut self, game_time: &GameTime) -> Result<(), Exception> {
        if self.handler.is_some() {
            self.handler.as_ref().unwrap().borrow_mut().on_update(game_time)?;
        }

        Ok(())
    }

    fn begin_draw(&self) -> Result<(), Exception> {
        if self.handler.is_some() {
            self.handler.as_ref().unwrap().borrow_mut().on_begin_draw()?;
        }

        Ok(())
    }

    fn draw(&mut self, game_time: &GameTime) -> Result<(), Exception> {
        if self.handler.is_some() {
            self.handler.as_ref().unwrap().borrow_mut().on_draw(game_time)?;
        }

        if self.graphics_device.is_none() {
            return Ok(());
        }

        self.graphics_device
            .unwrap_ref_or_default_exception()?
            .borrow()
            .present()?;

        Ok(())
    }

    fn end_draw(&mut self) -> Result<(), Exception> {
        if self.handler.is_some() {
            self.handler.as_ref().unwrap().borrow_mut().on_end_draw()?;
        }


        Ok(())
    }

    pub fn resize_window(&mut self, width: u32, height: u32) -> Result<(), Exception> {
        let gw_temp = self.game_window.unwrap_ref_or_default_exception()?;
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
        if !self.is_fixed_time_step {
            return;
        }

        self.platform.step_timer.target_elapsed_ticks = value.ticks as u64;
    }

    pub fn set_is_fixed_time_step(&mut self, value: bool) {
        self.is_fixed_time_step = value;
        self.platform.step_timer.is_fixed_time_step = value;
    }

}