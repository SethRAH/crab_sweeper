use ggez;
use ggez::{Context, GameResult};
use ggez::graphics::{Color};

use crate::ui_common::mouse_input_handler::{MouseInputHandler};

pub trait GameScreen {
    fn get_bg_color (&mut self) ->  GameResult<Color>;
    fn update (&mut self, ctx: &mut Context, mouse_input: &mut MouseInputHandler) -> GameResult<Option<Box<dyn GameScreen>>>;
    fn draw (&mut self, ctx: &mut Context) -> GameResult;
    fn init (&mut self) -> GameResult;
}