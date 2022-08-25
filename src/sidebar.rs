
use ggez::{GameResult, Context};
use ggez::graphics::*;
use glam::f32::Vec2;

use globals::*;
use text;


pub const SIDEBAR_WIDTH: f32 = 70.0;

#[derive(Debug)]
pub struct Sidebar {
    title: Text,
    color: Color,
    x: f32,
    bounty_amount: f32,
    life_amount: f32,
}

pub fn amount_to_dots(amount: f32) -> f32 {
    if amount < THRESHOLD1 {
        amount
    } else {
        THRESHOLD1 + (amount - THRESHOLD1).sqrt()
    }
    /*} else {
        THRESHOLD1 + (THRESHOLD2 - THRESHOLD1) / 2.0 + (amount - THRESHOLD2) / 4.0
    }*/
}

impl Sidebar {
    pub fn new(ctx: &mut Context, font: &Font, title: &'static str, color: Color, x: f32) -> GameResult<Sidebar> {
        Ok(Sidebar {
            title: Text::new(
                TextFragment::new(title)
                    .font(*font)
            ),
            color,
            x,
            bounty_amount: 0.0f32,
            life_amount: 0.0f32,
        })
    }

    pub fn update(&mut self, _ctx: &mut Context, bounty_amount: f32, life_amount: f32) {
        self.bounty_amount = bounty_amount;
        self.life_amount = life_amount;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(self.x, 0.0, SIDEBAR_WIDTH, WINDOW_HEIGHT as f32),
            self.color
        )?.draw(ctx, DrawParam::default())?;

        let center = Vec2::new(
            self.x + SIDEBAR_WIDTH / 2.0,
            WINDOW_HEIGHT as f32 - 50.0,
        );
        text::draw_centered_text(
            ctx,
            &self.title,
            center,
            0.0,
            DrawParam::default()
                .color(Color::from_rgb(0, 0, 0))
        )?;

        let meter_bounty_color = Color::from_rgb(15, 117, 188);
        let meter_life_color = Color::from_rgb(247, 148, 30);
        let meter_offset_x = 36.0f32;
        let meter_spacing_y = 14.0f32;
        let meter_radius = 9.0f32;
        let meter_bottom = Vec2::new(
            self.x + meter_offset_x,
            WINDOW_HEIGHT as f32 - 100.0,
        );

        let num_bounty_dots = amount_to_dots(self.bounty_amount.floor()) as usize;
        //let num_life_dots = (amount_to_dots(self.life_amount + self.bounty_amount) as usize) - num_bounty_dots;
        let num_life_dots = if self.bounty_amount > 0.0 {0} else
        {self.life_amount.floor() as usize};
        // let num_life_dots = (amount_to_dots(self.life_amount + self.bounty_amount) as usize) - num_bounty_dots;
        let num_dots_max = 34;

        let mut meter_cur = meter_bottom.clone();

        for _ in 0..num_life_dots.min(num_dots_max) {
            Mesh::new_circle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                meter_cur,
                meter_radius,
                2.0,
                meter_life_color
            )?.draw(ctx, DrawParam::default())?;
            meter_cur.y = meter_cur.y - meter_spacing_y;
        }

        for _ in num_life_dots..num_bounty_dots.min(num_dots_max) {
            Mesh::new_circle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                meter_cur,
                meter_radius,
                2.0,
                meter_bounty_color
            )?.draw(ctx, DrawParam::default())?;
            meter_cur.y = meter_cur.y - meter_spacing_y;
        }
        Ok(())
    }
}
