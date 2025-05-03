use crate::xna::csharp::{Exception, TimeSpan};
use crate::xna::framework::game::{Game, GameTime};
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, PeekMessageA, TranslateMessage, MSG, PM_REMOVE, WM_QUIT};

impl Game {
    pub fn exit(&self) -> Result<(), Exception> {
        self.game_window.as_ref().unwrap().close()
    }

    fn start_game_loop(&mut self) -> Result<(), Exception> {
        let mut msg = MSG::default();

        loop {
            unsafe{
                if PeekMessageA(&mut msg, Some(self.game_window.as_ref().unwrap().platform.hwnd), 0, 0, PM_REMOVE).as_bool() {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                } else {
                    self.tick()?;
                }
            }

            if msg.message == WM_QUIT{
                break;
            }
        }

        Ok(())
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
            self.update(&self.current_game_time)?;

            Ok(())
        };

        timer.tick(&mut lambda)?;

        self.platform.step_timer = timer;

        self.begin_draw()?;
        self.draw(&self.current_game_time)?;
        self.end_draw()?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        if self.platform.is_running {
            return Err(Exception::new("", None));
        }

        self.game_window.as_mut().unwrap().create()?;

        self.initialize()?;

        if self.graphics_device.is_none() {
            return Err(Exception::new("", None))
        }

        self.platform.is_running = true;
        self.begin_run()?;
        self.start_game_loop()?;

        Ok(())
    }

    fn initialize(&self) -> Result<(), Exception> {
        if self.platform.initialize_fn.is_some() {
            self.platform.initialize_fn.as_ref().unwrap()()?;
        }

        self.load_content()
    }

    fn load_content(&self) -> Result<(), Exception> {
        if self.platform.load_content_fn.is_some() {
            self.platform.load_content_fn.as_ref().unwrap()()?;
        }

        Ok(())
    }

    fn update(&self, game_time: &GameTime) -> Result<(), Exception> {
        if self.platform.update_fn.is_some() {
            self.platform.update_fn.unwrap()(game_time)?
        }

        Ok(())
    }

    fn begin_draw(&self)-> Result<(), Exception> {
        if self.platform.begin_fn.is_some() {
            self.platform.begin_fn.unwrap()()?
        }

        Ok(())
    }

    fn draw(&self, game_time: &GameTime)-> Result<(), Exception> {
        if self.platform.draw_fn.is_some() {
            self.platform.draw_fn.unwrap()(game_time)?
        }

        Ok(())
    }

    fn end_draw(&self)-> Result<(), Exception> {
        if self.platform.end_fn.is_some() {
            self.platform.end_fn.unwrap()()?
        }

        Ok(())
    }

    fn begin_run(&self)-> Result<(), Exception> {
        if self.platform.begin_run_fn.is_some() {
            self.platform.begin_run_fn.unwrap()()?
        }

        Ok(())
    }
}