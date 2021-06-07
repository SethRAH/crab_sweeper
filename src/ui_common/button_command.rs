use ggez;
use ggez::{GameResult};

use crate::screens::game_screen::{GameScreen};

pub trait ButtonCommand<T> where T : GameScreen {
    fn execute(&mut self, screen: &mut T)  -> GameResult;
    fn copy_command(&mut self) -> Box<dyn ButtonCommand<T>>;
}