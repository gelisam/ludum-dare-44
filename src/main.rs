extern crate core;
extern crate ggez;
extern crate rand;

use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font};
use ggez::timer;
use ggez::audio;

mod bg;
mod center;
mod globals;
mod text;
mod vector;

use globals::*;


#[derive(Debug)]
struct Assets {
    ambient: audio::Source,
    bg: bg::Assets,
    font: Font,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        ambient: audio::Source::new(ctx, "/ambient.ogg")?,
        bg: bg::load_assets(ctx)?,
        font,
    })
}


#[derive(Debug)]
struct Globals {
    assets: Assets,
    start_time: Duration,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        Ok(Globals {
            assets: load_assets(ctx)?,
            start_time: get_current_time(ctx),
        })
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.start_time = get_current_time(ctx);
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::R     => self.reset(ctx),
            _              => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        bg::draw_bg(ctx, &self.assets.bg)?;

        let msg = format!("{:#?}", get_current_time(ctx) - self.start_time);
        let text = graphics::Text::new(
            ctx,
            &msg,
            &self.assets.font
        )?;
        let center = graphics::Point2::new(
            WINDOW_WIDTH as f32 / 2.0,
            WINDOW_HEIGHT as f32 / 2.0,
        );
        text::draw_centered_text(ctx, &text, center, 0.0)?;

        graphics::present(ctx);
        timer::yield_now();

        Ok(())
    }
}

pub fn main() {
    let ctx = &mut Context::load_from_conf(
        GAME_NAME,
        "Michaelson Britt and Samuel Gélineau",
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
	globals.assets.ambient.play().unwrap();

    event::run(ctx, globals).unwrap();
}
