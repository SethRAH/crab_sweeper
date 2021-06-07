use ggez;
use ggez::{Context, GameResult};
use ggez::graphics::{Color};

use std::process;

use crate::screens::game_screen::{GameScreen};

use crate::ui_common::mouse_input_handler::{MouseInputHandler};


// pretty much an empty implementation to detect if we want to exit
pub struct ExitScreen {

}


impl GameScreen for ExitScreen {
    fn get_bg_color (&mut self) -> GameResult<Color> { 
        Ok(Color::from_rgb(0,0,0))
    }

    fn update (&mut self, _ctx: &mut Context, _mouse_input: &mut MouseInputHandler) -> GameResult<Option<Box<dyn GameScreen>>>{
        process::exit(0);
    }

    fn draw (&mut self, _ctx: &mut Context) -> GameResult{ Ok(()) }

    fn init (&mut self) -> GameResult{ Ok(()) }
}