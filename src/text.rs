use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, Point2, Text, Vector2};

use vector;


fn text_size(text: &Text) -> Vector2 {
    Vector2::new(text.width() as f32, text.height() as f32)
}

pub fn draw_centered_text(ctx: &mut Context, text: &Text, dest: Point2, rotation: f32) -> GameResult<()> {
    text.draw(ctx, dest - vector::rotate(text_size(text), rotation) / 2.0, rotation)
}
