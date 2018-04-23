use ggez::{GameResult, Context};
use ggez::graphics::{Point2, Text, Vector2};

use center::draw_centered;


fn text_size(text: &Text) -> Vector2 {
    Vector2::new(text.width() as f32, text.height() as f32)
}

pub fn draw_centered_text(ctx: &mut Context, text: &Text, dest: Point2, rotation: f32) -> GameResult<()> {
    draw_centered(ctx, text, text_size(text), dest, rotation)
}
