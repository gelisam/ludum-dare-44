use ggez::{GameResult, Context};
use ggez::graphics::{Image, Point2};

use globals::*;
use center::*;


#[derive(Debug)]
pub struct Assets {
    bomb3: Image,
    bomb2: Image,
    bomb1: Image,
    bomb0: Image,
    boom:  Image,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(
        Assets {
            bomb3: Image::new(ctx, "/bomb3.png")?,
            bomb2: Image::new(ctx, "/bomb2.png")?,
            bomb1: Image::new(ctx, "/bomb1.png")?,
            bomb0: Image::new(ctx, "/bomb0.png")?,
            boom:  Image::new(ctx, "/boom.png")?,
        }
    )
}


// 0..4
pub type FuseLength = usize;

#[derive(Clone, Copy, Debug)]
pub struct Bomb {
    pub fuse_length: FuseLength,
}

impl Bomb {
    pub fn new() -> Bomb {
        Bomb {
            fuse_length: 4
        }
    }

    pub fn trigger_chain_reaction(&mut self) {
        self.fuse_length = 0;
    }

    pub fn update(&mut self) {
        if self.fuse_length > 0 {
            self.fuse_length -= 1;
        }
    }

    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: Point2) -> GameResult<()> {
        let bomb_assets = [&assets.bomb0, &assets.bomb1, &assets.bomb2, &assets.bomb3];
        draw_centered(ctx, bomb_assets[self.fuse_length], image_size(), dest, 0.0)
    }
}

