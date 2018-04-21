extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics::{self, DrawMode, Point2, Text};

mod globals;
mod hex;
mod text;

use globals::*;


struct Assets {
    hello_world: Text,
    polygon_outline: hex::PolygonAsset,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = graphics::Font::default_font()?;

    Ok(Assets {
        hello_world: Text::new(ctx, "Hello, world!", &font)?,
        polygon_outline: hex::load_polygon_asset(ctx, DrawMode::Line(1.0))?,
    })
}

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
            &self.assets.polygon_outline,
        )?;
        text::draw_centered_text(
            ctx,
            &self.assets.hello_world,
            Point2::new(
                WINDOW_WIDTH  as f32 / 2.0,
                WINDOW_HEIGHT as f32 / 2.0,
            ),
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
