use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, DrawParam, Image};
use glam::f32::Vec2;

use vector;

pub fn draw_centered<D: Drawable>(
    ctx: &mut Context,
    drawable: &D,
    size: Vec2,
    dest: Vec2,
    rotation: f32,
    draw_param: DrawParam
) -> GameResult<()> {
    drawable.draw(
        ctx,
        draw_param
          .dest(dest - vector::rotate(size, rotation) / 2.0)
          .rotation(rotation)
    )
}

pub fn draw_centered_image(
    ctx: &mut Context,
    image: &Image,
    dest: Vec2,
    rotation: f32,
    draw_param: DrawParam
) -> GameResult<()> {
    draw_centered(
        ctx,
        image,
        Vec2::new(image.width() as f32, image.height() as f32),
        dest,
        rotation,
        draw_param
    )
}
