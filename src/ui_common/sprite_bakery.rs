use ggez::{graphics, Context, GameResult};
use ggez::graphics::{Image, DrawParam};
use ggez::graphics::spritebatch::SpriteBatch;

use std::collections::HashMap;

pub struct SpriteBakery {
    batches: HashMap<String, SpriteBatch>
}

impl SpriteBakery {
    pub fn new() -> Self {
        SpriteBakery { batches: HashMap::new()}
    }

    pub fn add_batch(&mut self, key: String, image: Image){        
        let new_batch = SpriteBatch::new(image);
        self.batches.entry(key).or_insert(new_batch);
    }

    pub fn add_param<P>(&mut self, key: String, param: P)
        where P: Into<DrawParam> {
            let batch = self.batches.get_mut(&key);
            if batch.is_some() {
                batch.unwrap().add(param);
            } 
        }
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        for (_key, batch) in self.batches.iter_mut() {
            graphics::draw(ctx, batch, DrawParam::new())?;
            batch.clear();
        }

        Ok(())
    }
}