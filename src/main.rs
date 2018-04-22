extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font};
use ggez::timer;

mod action;
mod bg;
mod car;
mod center;
mod globals;
mod hex;
mod map;
mod powerup;
mod text;
mod vector;

use globals::*;
use map::Map;


#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    hex: hex::Assets,
    map: map::Assets,
    powerup: powerup::Assets,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        bg: bg::load_assets(ctx)?,
        hex: hex::load_assets(ctx)?,
        map: map::load_assets(ctx, &font)?,
        powerup: powerup::load_assets(ctx, &font)?,
    })
}

#[derive(Debug)]
struct Mouse {
    pos_x: i32,
    pos_y: i32,
    mouse_down: bool,
}

impl Mouse {
    fn new() -> Mouse {
        Mouse{pos_x: 100, pos_y: 100, mouse_down: false}
    }
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    map: Map,
    mouse: Mouse,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        Ok(Globals {
            assets: load_assets(ctx)?,
            map: Map::load(),
            mouse: Mouse::new()
        })
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        // Game logic usually happen inside the while loop
        while timer::check_update_time(_ctx, DESIRED_FPS) {
            static mut FRAME_COUNT: i32 =  0;
            unsafe {
                FRAME_COUNT += 1;
                if FRAME_COUNT % 60 == 0 {
                    println!("Game in {}FPS, Execution time: {} frames, \
                        {} secs", DESIRED_FPS, FRAME_COUNT, FRAME_COUNT / 60);
                }
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.map.go_forward();
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        bg::draw_bg(ctx, &self.assets.bg)?;

        hex::draw_hex_grid(
            ctx,
            &self.assets.hex,
        )?;
        self.map.draw(ctx, &self.assets.map)?;

        graphics::present(ctx);
        timer::yield_now();

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
