extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics::{self, Point2, Text};

mod globals;
mod hex;
mod text;

use globals::*;


#[derive(Debug)]
struct Assets {
    hex: hex::Assets,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = graphics::Font::default_font()?;

    Ok(Assets {
        hex: hex::load_assets(ctx)?,
    })
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        Ok(Globals {
            assets: load_assets(ctx)?,
        })
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        hex::draw_hex_grid(
            ctx,
            &self.assets.hex,
        )?;

        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let ctx = &mut Context::load_from_conf(
        GAME_NAME,
        "Patrick Marchand, Samuel Gélineau, and Yen-Kuan Wu",
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
    event::run(ctx, globals).unwrap();
}
