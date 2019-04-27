use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, Image, Point2, Vector2};

use vector;

pub fn draw_centered<D: Drawable>(
    ctx: &mut Context,
    drawable: &D,
    size: Vector2,
    dest: Point2,
    rotation: f32,
) -> GameResult<()> {
    drawable.draw(ctx, dest - vector::rotate(size, rotation) / 2.0, rotation)
}

pub fn draw_centered_image(
    ctx: &mut Context,
    image: &Image,
    dest: Point2,
    rotation: f32
) -> GameResult<()> {
    draw_centered(ctx, image, Vector2::new(image.width() as f32, image.height() as f32), dest, rotation)
}
