use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, DrawParam, Image};
use mint::Point2;


#[derive(Debug)]
pub struct Assets {
    bg: Image,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(
        Assets {
            bg: Image::new(ctx, "/bg.png")?,
        }
    )
}

pub fn draw_bg(ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    assets.bg.draw(ctx, DrawParam::default())
}
