use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, Point2, Vector2};

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
