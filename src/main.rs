use ggez;
use ggez::conf;
use ggez::event::{self};
use ggez::graphics::{self};
use ggez::input::mouse::{MouseButton};
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use glam::*;

use std::path;

mod screens;
use screens::game_screen::{GameScreen};
use screens::splash_screen::{SplashScreen};

mod ui_common;
use ui_common::mouse_input_handler::{MouseInputHandler};

mod constants {
    pub const SCREEN_WIDTH: f32 = 800.0;
    pub const SCREEN_HEIGHT: f32 = 500.0;
    pub const FONT: &str = "/VT323-Regular.ttf";
}

struct GameState {
    screen:  Box<dyn GameScreen>,
    mouse_input_handler: MouseInputHandler
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        let initial_state = SplashScreen::new(ctx).unwrap();
        let mouse_input_handler = MouseInputHandler::new();
        GameState {
            screen: Box::new(initial_state),
            mouse_input_handler
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let next_screen = self.screen.update(ctx, &mut self.mouse_input_handler)?;
            if next_screen.is_some() {
                self.screen = next_screen.unwrap();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, self.screen.get_bg_color()?);

        self.screen.draw(ctx)?;

        graphics::present(ctx)?;
        
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y:f32){
        self.mouse_input_handler.record_button_click(button, x, y, true);
    }
    
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y:f32){
        self.mouse_input_handler.record_button_click(button, x, y, false);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32){
        self.mouse_input_handler.record_mouse_motion(x, y);
    }

}

pub fn main() -> GameResult {
    let resource_dir = path::PathBuf::from("./resources");

    let cb = ContextBuilder::new("Crab Sweeper", "sethrah")
        .window_setup(conf::WindowSetup::default().title("Crab Sweeper!"))
        .window_mode(conf::WindowMode::default().dimensions(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT))
        .add_resource_path(resource_dir);        

    let (mut ctx, mut events_loop) = cb.build()?;

    let mut game = GameState::new(&mut ctx);
    event::run(&mut ctx, &mut events_loop, &mut game)
}