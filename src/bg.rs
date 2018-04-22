use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, Image, Point2};


#[derive(Debug)]
pub struct Assets {
    bg: Image,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(
        Assets {
            bg: Image::new(ctx, "/bg.jpg")?,
        }
    )
}

pub fn draw_bg(ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    assets.bg.draw(ctx, Point2::new(0.0, 0.0), 0.0)
}
