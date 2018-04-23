use ggez::{GameResult, Context};
use ggez::graphics::{Image, Point2};

use globals::*;
use center::*;


#[derive(Debug)]
pub struct Assets {
    bomb3:       Image,
    bomb2:       Image,
    bomb1:       Image,
    bomb0:       Image,
    boom:        Image,
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


// 0..4 for Bomb, and 4..MAX_FUSE_LENGTH+1 for FutureBomb
pub type FuseLength = usize;

#[derive(Clone, Copy, Debug)]
pub struct Bomb {
    pub fuse_length: FuseLength,
}

impl Bomb {
    pub fn new(fuse_length: FuseLength) -> Bomb {
        Bomb {fuse_length}
    }

    #[allow(dead_code)]
    pub fn trigger_chain_reaction(self) -> Bomb {
        Bomb {
            fuse_length: 0,
            ..self
        }
    }

    pub fn decrement(self) -> Option<Bomb> {
        if self.fuse_length > 0 {
            Some(Bomb {
                fuse_length: self.fuse_length - 1,
                ..self
            })
        } else {
            None
        }
    }

    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: Point2) -> GameResult<()> {
        let bomb_assets = [&assets.bomb0, &assets.bomb1, &assets.bomb2, &assets.bomb3];
        draw_centered(ctx, bomb_assets[self.fuse_length], image_size(), dest, 0.0)
    }
}

