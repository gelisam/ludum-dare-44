use ggez::{GameResult, Context};
use ggez::graphics::*;

use globals::*;
use text;


pub const SIDEBAR_WIDTH: f32 = 150.0;

#[derive(Debug)]
pub struct Sidebar {
    title: Text,
    color: Color,
    x: f32,
}

impl Sidebar {
    pub fn new(ctx: &mut Context, font: &Font, title: &'static str, color: Color, x: f32) -> GameResult<Sidebar> {
        Ok(Sidebar {
            title: Text::new(ctx, title, font)?,
            color,
            x,
        })
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, self.color)?;
        rectangle(ctx, DrawMode::Fill, Rect::new(self.x, 0.0, SIDEBAR_WIDTH, WINDOW_HEIGHT as f32))?;

        set_color(ctx, Color::from_rgb(0, 0, 0))?;
        let center = Point2::new(
            self.x + SIDEBAR_WIDTH / 2.0,
            WINDOW_HEIGHT as f32 - 50.0,
        );
        text::draw_centered_text(ctx, &self.title, center, 0.0)?;

        Ok(())
    }
}
