use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, PeekMessageA, TranslateMessage, MSG, PM_REMOVE, WM_QUIT};
use crate::xna::csharp::{Exception, TimeSpan};
use crate::xna::framework::game::{Game, GameTime};
use crate::xna::platform::windows::game_window::WindowsGameWindow;
use crate::xna::platform::windows::StepTimer;

#[derive(Default, PartialEq, Clone)]
pub struct WindowsGame {
    pub is_running: bool,
    pub game_window: WindowsGameWindow,
    pub step_timer: StepTimer,

    pub base: Game,

    pub begin_run_fn: Option<fn() ->Result<(), Exception>>,
    pub update_fn: Option<fn(&GameTime) ->Result<(), Exception>>,
    pub draw_fn: Option<fn(&GameTime) ->Result<(), Exception>>,
    pub begin_fn: Option<fn() ->Result<(), Exception>>,
    pub end_fn: Option<fn() ->Result<(), Exception>>,
    pub initialize_fn: Option<fn() ->Result<(), Exception>>,
    pub load_content_fn: Option<fn() ->Result<(), Exception>>,
}

impl Game {
    pub fn create(&self) -> WindowsGame {
        WindowsGame {
            base: self.clone(),
            ..Default::default()
        }
    }
}

impl WindowsGame {
    pub fn exit(&self) -> Result<(), Exception> {
        self.game_window.close()
    }

    fn start_game_loop(&mut self) -> Result<(), Exception> {
        let mut msg = MSG::default();

        loop {
            unsafe{
                if PeekMessageA(&mut msg, Some(self.game_window.hwnd), 0, 0, PM_REMOVE).as_bool() {
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
        let mut timer = self.step_timer.clone();

        let mut lambda=  || -> Result<(), Exception> {
            let elapsed = self.step_timer.get_elapsed_seconds();
            let total = self.step_timer.get_total_seconds();
            let elapsed_time_span = TimeSpan::from_seconds(elapsed as i32)?;
            let total_time_span = TimeSpan::from_seconds(total as i32)?;
            self.base.current_game_time.elapsed_time = elapsed_time_span;
            self.base.current_game_time.total_time = total_time_span;
            self.update(&self.base.current_game_time)?;

            Ok(())
        };

        timer.tick(&mut lambda)?;

        self.step_timer = timer;

        self.begin_draw()?;
        self.draw(&self.base.current_game_time)?;
        self.end_draw()?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        if self.is_running {
            return Err(Exception::new("", None));
        }

        self.base.game_window.as_ref().unwrap().create()?;

        self.initialize()?;

        if self.base.graphics_device.is_none() {
            return Err(Exception::new("", None))
        }

        self.is_running = true;
        self.begin_run()?;
        self.start_game_loop()?;

        Ok(())
    }

    fn initialize(&self) -> Result<(), Exception> {
        if self.initialize_fn.is_some() {
            self.initialize_fn.as_ref().unwrap()()?;
        }

        self.load_content()
    }

    fn load_content(&self) -> Result<(), Exception> {
        if self.load_content_fn.is_some() {
            self.load_content_fn.as_ref().unwrap()()?;
        }

        Ok(())
    }

    fn update(&self, game_time: &GameTime) -> Result<(), Exception> {
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

    fn draw(&self, game_time: &GameTime)-> Result<(), Exception> {
        if self.draw_fn.is_some() {
            self.draw_fn.unwrap()(game_time)?
        }

        Ok(())
    }

    fn end_draw(&self)-> Result<(), Exception> {
        if self.end_fn.is_some() {
            self.end_fn.unwrap()()?
        }

        Ok(())
    }

    fn begin_run(&self)-> Result<(), Exception> {
        if self.begin_run_fn.is_some() {
            self.begin_run_fn.unwrap()()?
        }

        Ok(())
    }
}