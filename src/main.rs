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
mod checkpoint;
mod globals;
mod hex;
mod map;
mod powerup;
mod text;
mod vector;

use globals::*;
use car::Car;
use hex::HexPoint;
use checkpoint::*;
use map::{CellContents,Map};


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
struct Globals {
    assets: Assets,
    frame_count: i32,
    map: Map,
    car_position: HexPoint,
    car_checkpoint: Checkpoint,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        let mut map = Map::load();

        let car_position = HexPoint::new(CENTRAL_OBSTACLE_RADIUS+2, 0);
        map.insert(car_position, CellContents::Car(Car::new(forward(car_position))));

        Ok(Globals {
            assets: load_assets(ctx)?,
            frame_count: 0,
            map,
            car_position,
            car_checkpoint: 0,
        })
    }

    fn go_forward(&mut self) {
        self.map.remove(self.car_position);
        self.car_position += forward(self.car_position);
        self.map.insert(self.car_position, CellContents::Car(Car::new(forward(self.car_position))));

        self.car_checkpoint = update_checkpoint(self.car_checkpoint, self.car_position);
        println!(
            "section {:?}, checkpoint {:?}, lap {:?}",
            point_to_section(self.car_position),
            self.car_checkpoint,
            lap(self.car_checkpoint),
        );
    }

    fn go_backwards(&mut self) {
        self.map.remove(self.car_position);
        self.car_position += backward(self.car_position);
        self.map.insert(self.car_position, CellContents::Car(Car::new(forward(self.car_position))));

        self.car_checkpoint = update_checkpoint(self.car_checkpoint, self.car_position);
        println!(
            "section {:?}, checkpoint {:?}, lap {:?}",
            point_to_section(self.car_position),
            self.car_checkpoint,
            lap(self.car_checkpoint),
        );
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        // Game logic usually happen inside the while loop
        while timer::check_update_time(_ctx, DESIRED_FPS) {
            self.frame_count += 1;
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Up   => self.go_forward(),
            Keycode::Down => self.go_backwards(),
            _             => (),
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
