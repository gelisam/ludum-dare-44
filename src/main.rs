extern crate core;
extern crate ggez;
extern crate rand;

use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font};
use ggez::timer;

mod bg;
mod center;
mod channel;
mod globals;
mod hex;
mod text;
mod vector;

use globals::*;


#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    font: Font,
    hex: hex::Assets,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        bg: bg::load_assets(ctx)?,
        font,
        hex: hex::load_assets(ctx)?,
    })
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    start_time: Duration,
    bees: channel::Channel,
    birds: channel::Channel,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        Ok(Globals {
            assets: load_assets(ctx)?,
            start_time: get_current_time(ctx),
            bees: channel::Channel::new(ctx, "/bees.ogg")?,
            birds: channel::Channel::new(ctx, "/birds.ogg")?,
        })
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.start_time = get_current_time(ctx);
    }

    fn draw_left_sidebar(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::from_rgb(181, 208, 212))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(0.0, 0.0, 150.0, WINDOW_HEIGHT as f32))?;

        graphics::set_color(ctx, graphics::Color::from_rgb(0, 0, 0))?;
        let text = graphics::Text::new(ctx, "Bounty", &self.assets.font)?;
        let center = graphics::Point2::new(
            150.0 / 2.0,
            WINDOW_HEIGHT as f32 - 50.0,
        );
        text::draw_centered_text(ctx, &text, center, 0.0)?;

        graphics::set_color(ctx, graphics::Color::from_rgb(0, 0, 0))?;
        Ok(())
    }

    fn draw_right_sidebar(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::from_rgb(242, 240, 186))?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(WINDOW_WIDTH as f32 - 150.0, 0.0, 150.0, WINDOW_HEIGHT as f32))?;

        graphics::set_color(ctx, graphics::Color::from_rgb(0, 0, 0))?;
        let text = graphics::Text::new(ctx, "Life", &self.assets.font)?;
        let center = graphics::Point2::new(
            WINDOW_WIDTH as f32 - 150.0 / 2.0,
            WINDOW_HEIGHT as f32 - 50.0,
        );
        text::draw_centered_text(ctx, &text, center, 0.0)?;

        Ok(())
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.bees.update(ctx);
        self.birds.update(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Z      => self.bees.set_future_volume(ctx, Duration::from_millis(1000), 1.0),
            Keycode::X      => self.birds.set_future_volume(ctx, Duration::from_millis(1000), 1.0),
            Keycode::Escape => ctx.quit().unwrap(),
            _               => (),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Z     => self.bees.set_future_volume(ctx, Duration::from_millis(1000), 0.0),
            Keycode::X     => self.birds.set_future_volume(ctx, Duration::from_millis(1000), 0.0),
            Keycode::R     => self.reset(ctx),
            _              => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // must use white for drawing images, otherwise they get tinted
        graphics::set_color(ctx, graphics::Color::from_rgb(255, 255, 255))?;

        bg::draw_bg(ctx, &self.assets.bg)?;
        hex::draw_hex_grid(ctx, &self.assets.hex)?;
        self.draw_left_sidebar(ctx)?;
        self.draw_right_sidebar(ctx)?;

        graphics::present(ctx);
        timer::yield_now();

        Ok(())
    }
}

pub fn main() {
    let ctx = &mut Context::load_from_conf(
        GAME_NAME,
        "Michaelson Britt, Samuel Gélineau, Zhentao Li, Kyla Squires, and Farren Wang",
        ggez::conf::Conf {
            window_mode: ggez::conf::WindowMode {
                width:  WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                .. Default::default()
            },
            window_setup: ggez::conf::WindowSetup {
                title: GAME_NAME.to_owned(),
                .. Default::default()
            },
            .. Default::default()
        },
    ).unwrap();

    let globals = &mut Globals::new(ctx).unwrap();
	globals.bees.source.play().unwrap();
	globals.birds.source.play().unwrap();

    event::run(ctx, globals).unwrap();
}
