
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
    bounty_amount: f32,
}

impl Sidebar {
    pub fn new(ctx: &mut Context, font: &Font, title: &'static str, color: Color, x: f32) -> GameResult<Sidebar> {
        Ok(Sidebar {
            title: Text::new(ctx, title, font)?,
            color,
            x,
            bounty_amount: 0.0f32,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context, bounty_amount: f32) {
        self.bounty_amount = bounty_amount;
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

        let meter_color = Color::from_rgb(15, 117, 188);
        let meter_offset_x = 30.0f32;
        let meter_radius = 12.0f32;
        let meter_bottom = Point2::new(
            self.x + meter_offset_x,
            WINDOW_HEIGHT as f32 - 100.0,
        );

        let num_dots =
            match self.bounty_amount.floor() { //math::round::floor(bounty_amount)
                d if d < 0.0 => 0 as usize,
                d if d < 30.0 => d as usize,
                _   => 30 as usize,
            };

        set_color(ctx, meter_color)?;
        let mut meter_cur = meter_bottom.clone(); //Point2::new(meter_bottom);
        for _ in 0..num_dots {
            ggez::graphics::circle(
                ctx, 
                ggez::graphics::DrawMode::Fill, 
                meter_cur, 
                meter_radius, 
                1.0
            )?;
            meter_cur.y = meter_cur.y - 30.0;
        }
        Ok(())
    }
}
