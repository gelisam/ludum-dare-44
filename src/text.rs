use ggez::{GameResult, Context};
use ggez::graphics::{DrawParam, Text};
use glam::f32::Vec2;

use center::draw_centered;


fn text_size(ctx: &mut Context, text: &Text) -> Vec2 {
    Vec2::new(text.width(ctx) as f32, text.height(ctx) as f32)
}

pub fn draw_centered_text(
    ctx: &mut Context,
    text: &Text,
    dest: Vec2,
    rotation: f32,
    draw_param: DrawParam
) -> GameResult<()> {
    let size = text_size(ctx, text);
    draw_centered(ctx, text, size, dest, rotation, draw_param)
}
