use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, Point2, Text, Vector2};


fn text_size(text: &Text) -> Vector2 {
    Vector2::new(text.width() as f32, text.height() as f32)
}

pub fn draw_centered_text(ctx: &mut Context, text: &Text, o: Point2) -> GameResult<()> {
    text.draw(ctx, o - text_size(text) / 2.0, 0.0)?;
    Ok(())
}
