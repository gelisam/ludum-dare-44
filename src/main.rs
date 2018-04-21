extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics::{self, Font};

mod globals;
mod hex;
mod map;
mod text;
mod vector;

use globals::*;
use hex::HexPoint;
use map::CellContents;


#[derive(Debug)]
struct Assets {
    hex: hex::Assets,
    map: map::Assets,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        hex: hex::load_assets(ctx)?,
        map: map::load_assets(ctx, &font)?,
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

        let cells: Vec<CellContents> = vec!(
            CellContents::BonusBox,
            CellContents::Car,
            CellContents::CheckpointLine,
            CellContents::FinishLine,
            CellContents::Obstacle,
            CellContents::Wall,
        );
        let points: Vec<HexPoint> = HexPoint::new(0, 0).neighbours().to_vec();
        for i in 0..6 {
            cells[i].draw(ctx, &self.assets.map, points[i])?;
        }

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
