use ggez;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color};

use std::time::{Duration, Instant};

use crate::screens::game_screen::GameScreen;
use crate::screens::sweeper_screen::SweeperScreen;

use crate::ui_common::mouse_input_handler::MouseInputHandler;
use crate::constants;

struct AssetCollection {
    logo_one: graphics::Image,
    logo_two: graphics::Image,
}

impl AssetCollection {
    fn new(ctx: &mut Context) -> GameResult<AssetCollection> {
        let logo_one = graphics::Image::new(ctx, "/dev-logo-1.png")?;
        let logo_two = graphics::Image::new(ctx, "/dev-logo-2.png")?;

        Ok(AssetCollection{
            logo_one,
            logo_two
        })
    }
}

pub struct SplashScreen {
    assets: AssetCollection,
    first_time: Instant,
    show_logo_one: bool,
    show_logo_two: bool,
}

impl SplashScreen {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = AssetCollection::new(ctx)?;
        let now = Instant::now();
        Ok(SplashScreen{
            assets,
            first_time: now,
            show_logo_one: false,
            show_logo_two: false
        })
    }

    fn get_logo_one_pos(&mut self) -> glam::Vec2 {
        let x = (constants::SCREEN_WIDTH / 2.0) - (self.assets.logo_one.width() as f32 / 2.0);
        let y = (constants::SCREEN_HEIGHT / 2.0)  - ((self.assets.logo_one.height() as f32 + self.assets.logo_two.height() as f32) / 2.0);

        glam::Vec2::new(x,y)
    }

    
    fn get_logo_two_pos(&mut self) -> glam::Vec2 {
        let x = (constants::SCREEN_WIDTH / 2.0) - (self.assets.logo_two.width() as f32 / 2.0);
        let y = (constants::SCREEN_HEIGHT / 2.0) - ((self.assets.logo_one.height() as f32 + self.assets.logo_two.height() as f32) / 2.0) + self.assets.logo_one.height() as f32 - 12.0;

        glam::Vec2::new(x,y)
    }
}

impl GameScreen for SplashScreen {
    fn get_bg_color (&mut self) -> GameResult<Color> { 
        Ok(Color::from_rgb(255,255,255))
    }

    fn update (&mut self, _ctx: &mut Context, _mouse_input: &mut MouseInputHandler) -> GameResult<Option<Box<dyn GameScreen>>> {
        let elapsed = self.first_time.elapsed();
        let one_third_sec = Duration::from_millis(300);
        let one_sec = Duration::from_millis(1000);
        let three_sec = Duration::from_secs(3);

        self.show_logo_one = if elapsed > one_third_sec { true } else { false };
        self.show_logo_two = if elapsed > one_sec { true } else { false };

        if elapsed > three_sec {
            let next_screen = Box::new(SweeperScreen::new(_ctx)); //Box::new(ExitScreen{ });
            return Ok(Some(next_screen));
        }

        Ok(None)
    }

    fn draw (&mut self, ctx: &mut Context) -> GameResult{

        if self.show_logo_one {
            let dst = self.get_logo_one_pos();
            graphics::draw(ctx, &self.assets.logo_one, (dst,))?;
        }

        
        if self.show_logo_two {
            let dst = self.get_logo_two_pos();
            graphics::draw(ctx, &self.assets.logo_two, (dst,))?;
        }

        Ok(())
    }

    fn init (&mut self) -> GameResult{

        Ok(())
    }
}