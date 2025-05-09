use crate::xna::csharp::{Exception, TimeSpan};
use crate::xna::framework::game::{Game, GameTime};
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, PeekMessageA, TranslateMessage, MSG, PM_REMOVE, WM_QUIT};
use crate::xna::framework::graphics::GraphicsDevice;
use crate::xna::platform::windows::StepTimer;

impl Game {
    pub fn exit(&mut self) -> Result<(), Exception> {
        self.get_game_window()?.close()
    }

    fn start_game_loop(&mut self) -> Result<usize, Exception> {
        self.platform.step_timer = StepTimer::new();

        let mut msg = MSG::default();

        loop {
            let game_window = self.get_game_window()?;

            unsafe{
                if PeekMessageA(&mut msg, Some(game_window.platform.hwnd), 0, 0, PM_REMOVE).as_bool() {
                    let _ = TranslateMessage(&msg);
                    let _ = DispatchMessageA(&msg);
                } else {
                    self.tick()?;
                }
            }

            if msg.message == WM_QUIT{
                break;
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

    pub fn run(&mut self) -> Result<usize, Exception> {
        if self.platform.is_running {
            return Ok(1)
        }

        let mut game_window = self.get_game_window()?;
        game_window.create()?;

        self.initialize()?;

        if self.graphics_device.is_none() {
            return Err(Exception::new("The graphics device is invalid.", None))
        }

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

        self.get_graphics_device()?.present();

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
        let mut game_window = self.get_game_window()?;
        let windows_bounds = game_window.client_bounds();

        if windows_bounds.width != width as i32 || windows_bounds.height != height as i32 {
            game_window.width = width;
            game_window.height = height;

            game_window.update()?
        }

        Ok(())
    }

    pub fn attach_graphics_device(&mut self, device: Box<GraphicsDevice>) {
        self.graphics_device = Some(device);
    }

    pub fn run_one_frame(&mut self) -> Result<(), Exception> {
        self.tick()
    }

    pub fn reset_elapsed_time(&mut self) -> Result<(), Exception> {
        self.platform.step_timer.reset_elapsed_time()
    }

    pub fn target_elapsed_time(&mut self) {
        if !self.is_fixed_time_step{
            return;
        }

        let ticks = &self.target_elapsed_time.ticks;

        self.platform.step_timer.set_target_elapsed_ticks(*ticks as u64);
    }

    pub fn set_is_fixed_time_step(&mut self, value: bool) {
        self.is_fixed_time_step = value;
        self.platform.step_timer.set_fixed_time_step(value);
    }

}