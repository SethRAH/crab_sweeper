use getrandom;

use ggez;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam};

use oorandom::Rand32;

use std::collections::HashMap;
use std::convert::TryFrom;

use crate::constants;

use crate::screens::game_screen::{GameScreen};

use crate::ui_common::mouse_input_handler::{MouseInputHandler};
use crate::ui_common::button_command::{ButtonCommand};
use crate::ui_common::sprite_bakery::SpriteBakery;

const CLICKED_LONG_KEY: &str = "/Clicked_Tile_Long.png";
const CLICKED_SHORT_KEY: &str = "/Clicked_Tile.png";
const UNCLICKED_LONG_KEY: &str = "/Unclicked_Tile_Long.png";
const UNCLICKED_SHORT_KEY: &str = "/Unclicked_Tile.png";
const FLAG_MARKER_KEY: &str = "/Flag.png";
const QUESTION_MARKER_KEY: &str = "/Question.png";
const CRAB_KEY: &str = "/Crab.png";

struct AssetCollection {
    clicked_long_button: graphics::Image,
    clicked_short_button: graphics::Image,
    unclicked_long_button: graphics::Image,
    unclicked_short_button: graphics::Image,
    flag_marker: graphics::Image,
    question_marker: graphics::Image,
    crab_marker: graphics::Image,
    font: graphics::Font
}

impl AssetCollection {
    fn new(ctx: &mut Context) -> GameResult<AssetCollection> {
        let clicked_long_button = graphics::Image::new(ctx, CLICKED_LONG_KEY)?;
        let clicked_short_button = graphics::Image::new(ctx, CLICKED_SHORT_KEY)?;
        let unclicked_long_button = graphics::Image::new(ctx, UNCLICKED_LONG_KEY)?;
        let unclicked_short_button = graphics::Image::new(ctx, UNCLICKED_SHORT_KEY)?;
        let flag_marker = graphics::Image::new(ctx, FLAG_MARKER_KEY)?;
        let question_marker = graphics::Image::new(ctx, QUESTION_MARKER_KEY)?;
        let crab_marker = graphics::Image::new(ctx, CRAB_KEY)?;
        let font = graphics::Font::new(ctx, constants::FONT)?;

        Ok(AssetCollection{
            clicked_long_button,
            clicked_short_button,
            unclicked_long_button,
            unclicked_short_button,
            flag_marker,
            question_marker,
            crab_marker,
            font            
        })
    }
}

pub struct SweeperScreen {
    sprite_bakery: SpriteBakery,
    game_panel: GamePanel,
    game_board: GameBoard,
    assets: AssetCollection,
    rnd_seed: [u8; 8]
}

impl SweeperScreen {
    pub fn new(ctx: &mut Context) -> Self {
        let assets = AssetCollection::new(ctx).unwrap();
        let mut sprite_bakery = SpriteBakery::new();
        sprite_bakery.add_batch(CLICKED_LONG_KEY.to_string(), assets.clicked_long_button.clone());
        sprite_bakery.add_batch(CLICKED_SHORT_KEY.to_string(), assets.clicked_short_button.clone());
        sprite_bakery.add_batch(UNCLICKED_LONG_KEY.to_string(), assets.unclicked_long_button.clone());
        sprite_bakery.add_batch(UNCLICKED_SHORT_KEY.to_string(), assets.unclicked_short_button.clone());
        sprite_bakery.add_batch(FLAG_MARKER_KEY.to_string(), assets.flag_marker.clone());
        sprite_bakery.add_batch(QUESTION_MARKER_KEY.to_string(), assets.question_marker.clone());
        sprite_bakery.add_batch(CRAB_KEY.to_string(), assets.crab_marker.clone());
        let game_panel = GamePanel::new(CLICKED_LONG_KEY.to_string(), UNCLICKED_LONG_KEY.to_string());
        let mut rnd_seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut rnd_seed[..]).expect("Could not create RNG seed");
        let mut rng = Rand32::new(u64::from_ne_bytes(rnd_seed));
        let game_board = GameBoard::default(&mut rng, CLICKED_SHORT_KEY.to_string(), UNCLICKED_SHORT_KEY.to_string());

        SweeperScreen{ 
            sprite_bakery,
            game_panel,
            game_board,
            assets,
            rnd_seed
         }
    }

    pub fn reset(&mut self){        
        let mut rng = Rand32::new(u64::from_ne_bytes(self.rnd_seed));
        self.game_board = GameBoard::new(
            self.game_board.width,
            self.game_board.height,
            self.game_board.crab_ratio,
            &mut rng,
            self.game_board.clicked_image_key.clone(),
            self.game_board.unclicked_image_key.clone()
        )
    }

    pub fn change(&mut self, width: Option<u16>, height: Option<u16>, crab_ratio: Option<u16>){
        let effective_width = match width.is_some() { true => width.unwrap(), false => self.game_board.width };
        let effective_height = match height.is_some() { true => height.unwrap(), false => self.game_board.height };
        let effective_crab_ratio = match crab_ratio.is_some() { true => crab_ratio.unwrap(), false => self.game_board.crab_ratio };
        
        let mut rng = Rand32::new(u64::from_ne_bytes(self.rnd_seed));
        self.game_board = GameBoard::new(
            effective_width,
            effective_height,
            effective_crab_ratio,
            &mut rng,
            self.game_board.clicked_image_key.clone(),
            self.game_board.unclicked_image_key.clone()
        )
    }
}

impl GameScreen for SweeperScreen {
    fn get_bg_color (&mut self) -> GameResult<Color> { 
        Ok(Color::from_rgb(65, 146, 195))
    }

    fn update (&mut self, _ctx: &mut Context, mouse_input: &mut MouseInputHandler) -> GameResult<Option<Box<dyn GameScreen>>>{
        
        let command = self.game_panel.update(mouse_input)?;

        self.game_board.update(mouse_input)?;

        if command.is_some() {
            let mut unwrapped = command.unwrap();
            unwrapped.execute(self)?;
        }

        self.game_panel.num_flags = self.game_board.num_crabs() - self.game_board.num_flags();
        self.game_board.is_win();

        mouse_input.clear_stored_positions();

        Ok(None)
    }

    fn draw (&mut self, ctx: &mut Context) -> GameResult{
        
        // Game Panel Textures
        let draw_params = self.game_panel.draw_buttons(ctx);

        for (key, params) in &draw_params {
            for draw_param in params {
                self.sprite_bakery.add_param(key.clone(), draw_param.clone());
            }
        }
        // Game Board Tiles
        let draw_params = self.game_board.draw_tiles(ctx); 
        for (key, params) in &draw_params {
            for draw_param in params {
                self.sprite_bakery.add_param(key.clone(), draw_param.clone());
            }
        }

        self.sprite_bakery.draw(ctx)?;

        // Game Board Markers
        let draw_params = self.game_board.draw_markers(ctx); 
        for (key, params) in &draw_params {
            for draw_param in params {
                self.sprite_bakery.add_param(key.clone(), draw_param.clone());
            }
        }

        self.sprite_bakery.draw(ctx)?;

        //Text
        self.game_panel.draw_text(ctx, self.assets.font)?;
        self.game_board.draw_text(ctx, self.assets.font)?;

        Ok(())
    }

    fn init (&mut self) -> GameResult{

        Ok(())
    }
}


struct GamePanel {
    buttons: Vec<PanelButton>,
    num_flags: u16
}

impl GamePanel {
    pub fn new (long_button_clicked_image_key: String, long_button_unclicked_image_key: String) -> Self {
        let buttons = vec![
            PanelButton::new("Reset".to_string(), long_button_clicked_image_key.clone(), long_button_unclicked_image_key.clone(), 45.0, 97.0, Box::new(ResetCommand{})),
            PanelButton::new("10x10".to_string(), long_button_clicked_image_key.clone(), long_button_unclicked_image_key.clone(), 45.0, 161.0, Box::new(ChangeBoardCommand::new(Some(10), Some(10), None))),
            PanelButton::new("15x10".to_string(), long_button_clicked_image_key.clone(), long_button_unclicked_image_key.clone(), 45.0, 193.0, Box::new(ChangeBoardCommand::new(Some(15), Some(10), None))),
            PanelButton::new("15x15".to_string(), long_button_clicked_image_key.clone(), long_button_unclicked_image_key.clone(), 45.0, 225.0, Box::new(ChangeBoardCommand::new(Some(15), Some(15), None))),
            PanelButton::new("1:5".to_string(), long_button_clicked_image_key.clone(), long_button_unclicked_image_key.clone(), 45.0, 289.0, Box::new(ChangeBoardCommand::new(None, None, Some(5)))),
            PanelButton::new("1:8".to_string(), long_button_clicked_image_key.clone(), long_button_unclicked_image_key.clone(), 45.0, 321.0, Box::new(ChangeBoardCommand::new(None, None, Some(8)))),
            PanelButton::new("1:15".to_string(), long_button_clicked_image_key.clone(), long_button_unclicked_image_key.clone(), 45.0, 353.0, Box::new(ChangeBoardCommand::new(None, None, Some(10)))),
        ];
        GamePanel {
            buttons,
            num_flags: 0
        }
    }

    pub fn draw_buttons (&mut self, _ctx: &mut Context) -> HashMap<String, Vec<DrawParam>> {
        let mut result = HashMap::new();

        for button in &self.buttons {
            let key = match button.is_clicked {
                true => button.clicked_image_key.clone(),
                false => button.unclicked_image_key.clone()
            };
            let (x,y) = button.offset;
            let draw_param = DrawParam::new().dest(glam::Vec2::new(x,y));

            let params = result.entry(key).or_insert(vec![]);
            params.push(draw_param);
        }

        result
    }

    pub fn draw_text (&mut self, _ctx: &mut Context, font: graphics::Font) -> GameResult {
        // draw button group labels
        let panel_label = graphics::Text::new(("Panel", font, 32.0));
        let dim_label = graphics::Text::new(("Dim.", font, 32.0));
        let ratio_label = graphics::Text::new(("Crab Ratio", font, 32.0));

        graphics::draw(_ctx, &panel_label, DrawParam::new().dest(glam::Vec2::new(25.0, 62.0)))?;
        graphics::draw(_ctx, &dim_label, DrawParam::new().dest(glam::Vec2::new(25.0, 127.0)))?;
        graphics::draw(_ctx, &ratio_label, DrawParam::new().dest(glam::Vec2::new(25.0, 254.0)))?;
        
        let button_width = 96.0;
        let button_height = 32.0;
        
        // draw button text
        for button in &self.buttons {
            let button_label = graphics::Text::new((button.text.clone(), font, 24.0));
            let (x,y) = button.offset;
            let (tx, ty) = button_label.dimensions(_ctx);
            
            let label_x = ((button_width - tx as f32) / 2.0) + x;
            let label_y = ((button_height - ty as f32) / 2.0) + y;
            let draw_param = DrawParam::new().dest(glam::Vec2::new(label_x, label_y)).color(graphics::Color::from((48, 81, 130, 255)));
            graphics::draw(_ctx, &button_label, draw_param)?;
        }

        // draw flag counter text
        let flag_counter = self.num_flags.to_string();
        let flag_label = graphics::Text::new((flag_counter, font, 24.0));
        let (cx, cy) = flag_label.dimensions(_ctx);
        let label_x = ((button_width - cx as f32) / 2.0) + 45.0;
        let label_y = ((button_height - cy as f32) / 2.0) + 390.0;
        let draw_param = DrawParam::new().dest(glam::Vec2::new(label_x, label_y));
        graphics::draw(_ctx, &flag_label, draw_param)?;

        Ok(())
    }

    pub fn update(&mut self, mouse_input: &mut MouseInputHandler) -> GameResult<Option<Box<dyn ButtonCommand<SweeperScreen>>>>{
        let mut result: Option<Box<dyn ButtonCommand<SweeperScreen>>> = None; 
        for button in self.buttons.iter_mut() {
            let (x, y) = button.offset;
            let (width, height) = (96.0, 32.0);
            let does_intersect = Self::does_intersect(mouse_input.x, mouse_input.y, x, y, width, height);
            if (mouse_input.left_button_handler.is_initial_click || mouse_input.left_button_handler.is_held) && does_intersect {
                button.set_click(true);
            } else {
                button.set_click(false);                
            }

            if mouse_input.left_button_handler.last_release_position.is_some() && does_intersect {
                result = Some(button.copy_command());
            }
        }

        Ok(result)
    }

    fn does_intersect(mouse_x: f32, mouse_y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
        let rect_right = rect_x + rect_width;
        let rect_bottom = rect_y + rect_height;

        if mouse_x >= rect_x && mouse_x <= rect_right && mouse_y >= rect_y && mouse_y <= rect_bottom {
            return true;
        }

        false
    }

}

struct PanelButton {
    text: String,
    clicked_image_key: String,
    unclicked_image_key: String,
    offset: (f32, f32),
    command: Box<dyn ButtonCommand<SweeperScreen>>,
    is_clicked: bool
}

impl PanelButton {
    pub fn new(text: String, clicked_image_key: String, 
        unclicked_image_key: String, offset_x: f32, offset_y: f32, command: Box<dyn ButtonCommand<SweeperScreen>>) -> Self {
        let offset = (offset_x, offset_y);
        PanelButton{
            text,
            clicked_image_key,
            unclicked_image_key,
            offset,
            command,
            is_clicked: false
        }
    }

    pub fn set_click(&mut self, is_clicked: bool) { self.is_clicked = is_clicked; }

    pub fn copy_command(&mut self) -> Box<dyn ButtonCommand<SweeperScreen>> {
        self.command.copy_command()
    }
}

struct ResetCommand {}
impl ButtonCommand<SweeperScreen> for ResetCommand {
    fn execute(&mut self, screen: &mut SweeperScreen)  -> GameResult {
        screen.reset();
        Ok(())
    }

    fn copy_command(&mut self) -> Box<dyn ButtonCommand<SweeperScreen>> {
        Box::new(ResetCommand{})
    }
}

struct ChangeBoardCommand {
    width: Option<u16>,
    height: Option<u16>,
    crab_ratio: Option<u16>
}

impl ChangeBoardCommand {
    pub fn new(width: Option<u16>, height: Option<u16>, crab_ratio: Option<u16>) -> Self {
        ChangeBoardCommand {
            width,
            height,
            crab_ratio
        }
    }
}

impl ButtonCommand<SweeperScreen> for ChangeBoardCommand {
    fn execute(&mut self, screen: &mut SweeperScreen)  -> GameResult {
        screen.change(self.width, self.height, self.crab_ratio);
        Ok(())
    }
    
    fn copy_command(&mut self) -> Box<dyn ButtonCommand<SweeperScreen>> {
        Box::new(ChangeBoardCommand { 
            width: self.width, 
            height: self.height, 
            crab_ratio: self.crab_ratio, 
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FlagMarker {
    NONE = 0,
    FLAGGED = 1,
    QUESTIONED = 2
}

impl std::ops::AddAssign for FlagMarker {
    fn add_assign(&mut self, other: Self) {
       let new_me = *self as u8 + other as u8;
       let newer_me  = FlagMarker::try_from(new_me);
       *self = newer_me.unwrap_or(FlagMarker::NONE);
    }
}

impl TryFrom<u8> for FlagMarker {
    type Error =  &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let modded = value % 3;
        Ok(match modded {
            0 => FlagMarker::NONE,
            1 => FlagMarker::FLAGGED,
            2 => FlagMarker::QUESTIONED,
            _ => FlagMarker::NONE            
        })
    }
}

impl std::convert::TryFrom<FlagMarker> for u8 {
    type Error =  &'static str;

    fn try_from(value: FlagMarker) -> Result<Self, Self::Error> {
        Ok(match value {
            FlagMarker::NONE => 0,
            FlagMarker::FLAGGED => 1,
            FlagMarker::QUESTIONED => 2    
        })
    }
}


struct GameBoard {
    width: u16,
    height: u16,
    crab_ratio: u16,
    tile_size: f32,
    px_top: f32,
    px_left: f32,

    game_over: bool,
    win: bool,

    is_uncovered: [bool; 225],
    is_crab: [bool; 225],
    adjacency: [u8; 225],
    flag_marker: [FlagMarker; 225],

    clicked_image_key: String,
    unclicked_image_key: String
}

impl GameBoard {
    pub fn default(rand:&mut Rand32, clicked_image_key: String, unclicked_image_key: String) -> Self {
        let width = 10;
        let height = 10;
        let tile_size = 32.0;

        let (px_left, px_top) = GameBoard::get_offset(475.0, 250.0, width, height, tile_size);
        
        let mut board = GameBoard {
            width,
            height,
            crab_ratio: 10,
            tile_size,
            px_left,
            px_top,
            clicked_image_key,
            unclicked_image_key,
            game_over: false,
            win: false,

            is_uncovered: [false; 225],
            is_crab: [false; 225],
            adjacency: [0; 225],
            flag_marker: [FlagMarker::NONE; 225],
        };

        board.init(rand);

        board
    }

    pub fn new(width: u16, height: u16, crab_ratio: u16, rand:&mut Rand32, clicked_image_key: String, unclicked_image_key: String) -> Self {
        let tile_size = 32.0;

        let (px_left, px_top) = GameBoard::get_offset(475.0, 250.0, width, height, tile_size);
        
        let mut board = GameBoard {
            width,
            height,
            crab_ratio,
            tile_size,
            px_top,
            px_left,
            clicked_image_key,
            unclicked_image_key,
            game_over: false,
            win: false,
            is_uncovered: [false; 225],
            is_crab: [false; 225],
            adjacency: [0; 225],
            flag_marker: [FlagMarker::NONE; 225]
        };

        board.init(rand);

        board
    }

    fn init(&mut self, rand: &mut Rand32) {
        let board_size = (self.height * self.width) as usize;

        for i in 0..board_size {
            let crab_int = rand.rand_range(0..self.crab_ratio as u32);
            self.is_crab[i] = match crab_int { 0 => true, _ => false};
        }

        // loop through again and increment adjacency counter everytime you encouter a crab
        for i in 0..board_size {
            let mut count: u8 = 0;
            let (x, y) = self.index_to_coordinates(i);
            let right_bounds = self.width - 1;
            let bottom_bounds = self.height - 1;

            //NW
            if x > 0 && y > 0 && self.is_crab[self.coordinates_to_index(x-1, y-1)] { count += 1; }
            //N
            if y > 0 && self.is_crab[self.coordinates_to_index(x, y-1)] { count += 1; }
            //NE
            if x < right_bounds && y > 0 && self.is_crab[self.coordinates_to_index(x+1, y-1)] { count += 1; }
            //E
            if x < right_bounds && self.is_crab[self.coordinates_to_index(x+1, y)] { count += 1; }
            //SE
            if x < right_bounds && y < bottom_bounds && self.is_crab[self.coordinates_to_index(x+1, y+1)] { count += 1; }
            //S
            if y < bottom_bounds && self.is_crab[self.coordinates_to_index(x, y+1)] { count += 1; }
            //SW
            if x > 0 && y < bottom_bounds && self.is_crab[self.coordinates_to_index(x-1, y+1)] { count += 1; }
            //W
            if x > 0 && self.is_crab[self.coordinates_to_index(x-1, y)] { count += 1; }

            self.adjacency[i] = count;
        }

    }

    pub fn draw_tiles (&mut self, _ctx: &mut Context) -> HashMap<String, Vec<DrawParam>> {
        let mut result = HashMap::new();
        let board_size = (self.height * self.width) as usize;

        for i in 0..board_size {
            let (x,y) = self.index_to_coordinates(i);
            let px = self.px_left + x as f32 * self.tile_size;
            let py = self.px_top + y as f32 * self.tile_size;

            let key = match self.is_uncovered[i] { true => self.clicked_image_key.clone(), false => self.unclicked_image_key.clone()};
            
            let draw_param = DrawParam::new().dest(glam::Vec2::new(px,py));

            let params = result.entry(key).or_insert(vec![]);
            params.push(draw_param);
        }

        result
    }

    pub fn draw_markers (&mut self, _ctx: &mut Context) -> HashMap<String, Vec<DrawParam>> {
        let mut result = HashMap::new();
        let board_size = (self.height * self.width) as usize;

        for i in 0..board_size {
            let (x,y) = self.index_to_coordinates(i);
            let px = self.px_left + x as f32 * self.tile_size;
            let py = self.px_top + y as f32 * self.tile_size;
            let draw_param = DrawParam::new().dest(glam::Vec2::new(px,py));

            if self.is_uncovered[i] && self.is_crab[i] {
                let params = result.entry(String::from(CRAB_KEY)).or_insert(vec![]);
                params.push(draw_param);
            } else {
                match self.flag_marker[i] {
                    FlagMarker::FLAGGED => {
                        let params = result.entry(String::from(FLAG_MARKER_KEY)).or_insert(vec![]);
                        params.push(draw_param);
                    }                
                    FlagMarker::QUESTIONED => {
                        let params = result.entry(String::from(QUESTION_MARKER_KEY)).or_insert(vec![]);
                        params.push(draw_param);
                    }
                    _ => {}
                }
            }
        }

        result
    }

    pub fn draw_text (&mut self, ctx: &mut Context, font: graphics::Font) -> GameResult {
        // draw adjacency numbers
        let board_size = (self.height * self.width) as usize;
        for i in 0..board_size {
            let (x,y) = self.index_to_coordinates(i);
            let px = self.px_left + x as f32 * self.tile_size;
            let py = self.px_top + y as f32 * self.tile_size;
            
            if self.is_uncovered[i] && !self.is_crab[i] && self.adjacency[i] > 0 {
                let adjacency_label = graphics::Text::new((self.adjacency[i].to_string(), font, 24.0));
                let (tx, ty) = adjacency_label.dimensions(ctx);
                let label_x = ((32.0 - tx as f32) / 2.0) + px;
                let label_y = ((32.0 - ty as f32) / 2.0) + py;
                let draw_param = DrawParam::new().dest(glam::Vec2::new(label_x, label_y)).color(graphics::Color::from((48, 81, 130, 255)));
                graphics::draw(ctx, &adjacency_label, draw_param)?;
            }
        }

        //draw winning or gameover text
        if self.game_over {
            let board_width = self.width as f32 * self.tile_size;
            let board_height = self.height as f32 * self.tile_size;

            let game_over_text = match self.win { true => "#WINNING", false => "GAME OVER"};
            let game_over_label = graphics::Text::new((game_over_text, font, 72.0));
            let (tx, ty) = game_over_label.dimensions(ctx);
            let label_x = ((board_width - tx as f32) / 2.0) + self.px_left;
            let label_y = ((board_height - ty as f32) / 2.0) + self.px_top;
            let draw_param = DrawParam::new().dest(glam::Vec2::new(label_x, label_y));
            graphics::draw(ctx, &game_over_label, draw_param)?;
        }

        Ok(())
    }

    pub fn update(&mut self, mouse_input: &mut MouseInputHandler) -> GameResult {
        if mouse_input.left_button_handler.last_release_position.is_some() {
            let position = mouse_input.left_button_handler.last_release_position.unwrap();
            
            let index = self.mouse_input_to_tile_index(position[0], position[1]);
            if index.is_some() {
                let i = index.unwrap();
                self.uncover_tiles(i);
                if self.is_crab[i] {
                    self.game_over = true;
                    self.win = false;
                    self.reveal_bombs();
                }
            }
        }

        if mouse_input.right_button_handler.last_release_position.is_some() {
            let position = mouse_input.right_button_handler.last_release_position.unwrap();
            
            let index = self.mouse_input_to_tile_index(position[0], position[1]);
            if index.is_some() {
                let i = index.unwrap();
                self.flag_marker[i] += FlagMarker::try_from(1).unwrap_or(FlagMarker::NONE);
            }
        }


        Ok(())
    }

    fn does_intersect(mouse_x: f32, mouse_y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
        let rect_right = rect_x + rect_width;
        let rect_bottom = rect_y + rect_height;

        if mouse_x >= rect_x && mouse_x <= rect_right && mouse_y >= rect_y && mouse_y <= rect_bottom {
            return true;
        }

        false
    }

    fn coordinates_to_index(&mut self, x: u16, y: u16) -> usize{
        if x > self.width - 1 {
            return 0;
        }

        return ((y * self.width) + x) as usize;
    }

    fn index_to_coordinates(&mut self, i: usize) -> (u16, u16){
        (i as u16 % self.width, i as u16 / self.width)
    }

    fn mouse_input_to_tile_index(&mut self, mouse_x: f32, mouse_y: f32) -> Option<usize> {        
        let (px_board_width, px_board_height) = (self.width as f32 * self.tile_size, self.height as f32 * self.tile_size);

        if GameBoard::does_intersect(mouse_x, mouse_y, self.px_left, self.px_top, px_board_width, px_board_height) {
            let (rel_x, rel_y) = (mouse_x - self.px_left, mouse_y - self.px_top);
            let x = (rel_x / self.tile_size).floor() as u16;
            let y = (rel_y / self.tile_size).floor() as u16;

            return Some(self.coordinates_to_index(x, y));
        }

        None
    }

    fn reveal_bombs(&mut self) {
        let size = self.height as usize * self.width as usize;
        for i in 0..size {
            if self.is_crab[i] {
                self.is_uncovered[i] = true;
            }

        }
    }

    fn uncover_tiles(&mut self, clicked_index: usize) {
        let mut already_expanded: Vec<usize> = Vec::new();
        let mut expandable_indices: Vec<usize> = Vec::new();
        self.is_uncovered[clicked_index] = true;

        if !self.is_crab[clicked_index] && self.adjacency[clicked_index] == 0 {
            expandable_indices.push(clicked_index);
        }

        while !expandable_indices.is_empty() {
            let mut next_expandable_indices: Vec<usize> = Vec::new();

            for i in expandable_indices.iter_mut() {
                let (ix, iy) = self.index_to_coordinates(*i);
                let right_bounds = self.width - 1;
                let bottom_bounds = self.height - 1;

                //NW
                if ix > 0 && iy > 0  {
                    let nw = self.coordinates_to_index(ix-1, iy-1);
                    self.is_uncovered[nw] = true;
                    if !self.is_crab[nw] && self.adjacency[nw] == 0 && !already_expanded.contains(&nw){
                        already_expanded.push(nw);
                        next_expandable_indices.push(nw);
                    }
                }
                //N
                if iy > 0 {
                    let n = self.coordinates_to_index(ix, iy-1);
                    self.is_uncovered[n] = true;
                    if !self.is_crab[n] && self.adjacency[n] == 0 && !already_expanded.contains(&n){
                        already_expanded.push(n);
                        next_expandable_indices.push(n);
                    }
                }
                //NE
                if ix < right_bounds && iy > 0 {
                    let ne = self.coordinates_to_index(ix+1, iy-1);
                    self.is_uncovered[ne] = true;
                    if !self.is_crab[ne] && self.adjacency[ne] == 0 && !already_expanded.contains(&ne){
                        already_expanded.push(ne);
                        next_expandable_indices.push(ne);
                    }
                }
                //E
                if ix < right_bounds {
                    let e = self.coordinates_to_index(ix+1, iy);
                    self.is_uncovered[e] = true;
                    if !self.is_crab[e] && self.adjacency[e] == 0 && !already_expanded.contains(&e){
                        already_expanded.push(e);
                        next_expandable_indices.push(e);
                    }
                }
                //SE
                if ix < right_bounds && iy < bottom_bounds {
                    let se = self.coordinates_to_index(ix+1, iy+1);
                    self.is_uncovered[se] = true;
                    if !self.is_crab[se] && self.adjacency[se] == 0 && !already_expanded.contains(&se){
                        already_expanded.push(se);
                        next_expandable_indices.push(se);
                    }
                }
                //S
                if iy < bottom_bounds {
                    let s = self.coordinates_to_index(ix, iy+1);
                    self.is_uncovered[s] = true;
                    if !self.is_crab[s] && self.adjacency[s] == 0 && !already_expanded.contains(&s){
                        already_expanded.push(s);
                        next_expandable_indices.push(s);
                    }
                }
                //SW
                if ix > 0 && iy < bottom_bounds {
                    let sw = self.coordinates_to_index(ix-1, iy+1);
                    self.is_uncovered[sw] = true;
                    if !self.is_crab[sw] && self.adjacency[sw] == 0 && !already_expanded.contains(&sw){
                        already_expanded.push(sw);
                        next_expandable_indices.push(sw);
                    }
                }
                //W
                if ix > 0 {
                    let w = self.coordinates_to_index(ix-1, iy);
                    self.is_uncovered[w] = true;
                    if !self.is_crab[w] && self.adjacency[w] == 0 && !already_expanded.contains(&w){
                        already_expanded.push(w);
                        next_expandable_indices.push(w);
                    }
                }
            }

            expandable_indices = next_expandable_indices;
        }
    }

    pub fn num_crabs(&mut self) -> u16 {
        let mut counter = 0;
        let board_size = self.width * self.height;
        for i in 0..board_size as usize {
            if self.is_crab[i] {
                counter += 1;
            }
        }
        counter
    }

    pub fn num_flags(&mut self) -> u16 {
        let mut counter = 0;
        let board_size = self.width * self.height;
        for i in 0..board_size as usize {
            if self.flag_marker[i] == FlagMarker::FLAGGED {
                counter += 1;
            }
        }
        counter
    }

    pub fn is_win(&mut self) -> bool {
        let mut winning = true;
        let board_size = (self.width * self.height) as usize;
        let mut i:usize = 0;
        while i < board_size && winning {
            winning = self.is_uncovered[i] ^ self.is_crab[i];

            i += 1;
        }

        if winning {
            self.game_over = true;
            self.win = true;
        }

        winning
    }

    pub fn get_offset(center_x: f32, center_y: f32, width: u16, height: u16, tile_size: f32) -> (f32, f32) {
        let px_width = width as f32 * tile_size;
        let px_height = height as f32 * tile_size;

        let x = center_x - px_width / 2.0;
        let y = center_y - px_height / 2.0;

        (x,y)
    }
}


