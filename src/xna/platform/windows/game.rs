use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, PeekMessageA, TranslateMessage, MSG, PM_REMOVE, WM_QUIT};
use crate::xna::csharp::{Exception, TimeSpan};
use crate::xna::framework::game::{Game, GameTime};
use crate::xna::platform::windows::game_window::WindowsGameWindow;
use crate::xna::platform::windows::StepTimer;

#[derive(PartialEq, Clone)]
pub struct WindowsGame {
    is_running: bool,

    pub step_timer: StepTimer,
    pub game_window: WindowsGameWindow,

    pub base: Game,

    pub update_fn: Option<fn(&GameTime) ->Result<(), Exception>>,
    pub draw_fn: Option<fn(&GameTime) ->Result<(), Exception>>,
    pub begin_fn: Option<fn() ->Result<(), Exception>>,
    pub end_fn: Option<fn() ->Result<(), Exception>>,
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

    fn start_game_loop(&self) -> Result<(), Exception> {
        let mut msg = MSG::default();

        loop {
            unsafe{
                if PeekMessageA(&mut msg, Some(self.game_window.hwnd), 0, 0, PM_REMOVE) {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                } else {
                    tick();
                }
            }

            if msg.message == WM_QUIT{
                break;
            }
        }
    }

    fn tick(&mut self) -> Result<(), Exception> {
        let lambda = || -> Result<(), Exception> {
            let elapsed = self.step_timer.get_elapsed_seconds();
            let total = self.step_timer.get_total_seconds();
            let elapsed_time_span = TimeSpan::from_seconds(elapsed as i32)?;
            let total_time_span = TimeSpan::from_seconds(total as i32)?;
            self.base.current_game_time.elapsed_time = elapsed_time_span;
            self.base.current_game_time.total_time = total_time_span;
            self.update(&self.base.current_game_time)?;

            Ok(())
        };

        self.step_timer.tick(&lambda)?;

        self.begin_draw()?;
        self.draw(&self.base.current_game_time)?;
        self.end_draw()?;

        Ok(())
    }

    pub fn run(&self) -> Result<(i32), Exception> {
        if self.is_running {
            return Ok(1)
        }


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

    fn end_draw<F>(&self)-> Result<(), Exception> {
        if self.end_fn.is_some() {
            self.end_fn.unwrap()()?
        }

        Ok(())
    }
}