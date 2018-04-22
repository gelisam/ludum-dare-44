extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font};
use ggez::timer;

mod ai;
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

use ai::Ai;
use globals::*;
use hex::HexPoint;
use map::Map;
use racer::Racer;


#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    car: car::Assets,
    hex: hex::Assets,
    map: map::Assets,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        bg: bg::load_assets(ctx)?,
        car: car::load_assets(ctx)?,
        hex: hex::load_assets(ctx)?,
        map: map::load_assets(ctx, &font)?,
    })
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    exec_time: f32,
    map: Map,
    car3: Ai,
    car2: Ai,
    car1: Racer,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        let mut map = Map::load();

        let car3 = Racer::new(3, HexPoint::new(CENTRAL_OBSTACLE_RADIUS+1, 0));
        car3.insert(&mut map);

        let car2 = Racer::new(2, HexPoint::new(CENTRAL_OBSTACLE_RADIUS+2, 0));
        car2.insert(&mut map);

        let car1 = Racer::new(1, HexPoint::new(CENTRAL_OBSTACLE_RADIUS+3, 0));
        car1.insert(&mut map);

        Ok(Globals {
            assets: load_assets(ctx)?,
            exec_time: 0.0,
            map,
            car3: Ai::new(car3),
            car2: Ai::new(car2),
            car1,
        })
    }

    fn set_car3(&mut self, car3: Ai) {
        self.car3.racer.remove(&mut self.map);
        self.car3 = car3;
        self.car3.racer.insert(&mut self.map);
    }

    fn set_car2(&mut self, car2: Ai) {
        self.car2.racer.remove(&mut self.map);
        self.car2 = car2;
        self.car2.racer.insert(&mut self.map);
    }

    fn set_car1(&mut self, car1: Racer) {
        self.car1.remove(&mut self.map);
        self.car1 = car1;
        self.car1.insert(&mut self.map);
    }

    fn turn_left(&mut self) {
        let car1 = self.car1.turn_left();
        self.set_car1(car1)
    }

    fn turn_right(&mut self) {
        let car1 = self.car1.turn_right();
        self.set_car1(car1)
    }

    fn go_forward(&mut self) {
        if let Some(car1) = self.car1.go_forward(&self.map) {
            self.set_car1(car1);
        }
    }

    fn go_backwards(&mut self) {
        if let Some(car1) = self.car1.go_backwards(&self.map) {
            self.set_car1(car1);
        }
    }

    fn go_back_to_checkpoint(&mut self) {
        let car1 = self.car1.go_back_to_checkpoint(&self.map);
        self.set_car1(car1);
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
        let action3 = self.car3.next_action();
        let car3 = self.car3.perform_action(action3, &mut self.map);
        self.set_car3(car3);

        let action2 = self.car2.next_action();
        let car2 = self.car2.perform_action(action2, &mut self.map);
        self.set_car2(car2);

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
        self.map.draw(ctx, &self.assets.map, &self.assets.car)?;

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
