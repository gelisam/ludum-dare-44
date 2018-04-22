extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font};
use ggez::timer;

mod bg;
mod car;
mod center;
mod checkpoint;
mod globals;
mod hex;
mod map;
mod racer;
mod text;
mod vector;

use globals::*;
use hex::HexPoint;
use map::Map;
use racer::Racer;


#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    hex: hex::Assets,
    map: map::Assets,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        bg: bg::load_assets(ctx)?,
        hex: hex::load_assets(ctx)?,
        map: map::load_assets(ctx, &font)?,
    })
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    exec_time: f32,
    map: Map,
    player: Racer,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        let mut map = Map::load();

        let player = Racer::new(1, HexPoint::new(CENTRAL_OBSTACLE_RADIUS+2, 0));
        player.insert(&mut map);

        Ok(Globals {
            assets: load_assets(ctx)?,
            exec_time: 0.0,
            map,
            player,
        })
    }

    fn turn_left(&mut self) {
        self.player.remove(&mut self.map);
        self.player = self.player.turn_left();
        self.player.insert(&mut self.map);
    }

    fn turn_right(&mut self) {
        self.player.remove(&mut self.map);
        self.player = self.player.turn_right();
        self.player.insert(&mut self.map);
    }

    fn go_forward(&mut self) {
        self.player.remove(&mut self.map);
        self.player = self.player.go_forward();
        self.player.insert(&mut self.map);
    }

    fn go_backwards(&mut self) {
        self.player.remove(&mut self.map);
        self.player = self.player.go_backwards();
        self.player.insert(&mut self.map);
    }

    fn go_back_to_checkpoint(&mut self) {
        self.player.remove(&mut self.map);
        self.player = self.player.go_back_to_checkpoint(&self.map);
        self.player.insert(&mut self.map);
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        // Game logic usually happen inside the while loop
        while timer::check_update_time(_ctx, DESIRED_FPS) {
            self.exec_time += 1.0 / DESIRED_FPS as f32;
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Left  => self.turn_left(),
            Keycode::Right => self.turn_right(),
            Keycode::Up    => self.go_forward(),
            Keycode::Down  => self.go_backwards(),
            Keycode::R     => self.go_back_to_checkpoint(),
            _              => (),
        }
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
