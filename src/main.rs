extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font};
use ggez::timer;

mod ai;
mod animation;
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
use animation::*;
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


#[derive(Clone, Copy, Debug)]
enum State {
    WaitingForInput,
    WaitingForAnimation(TranslationAnimation, Racer),
}


#[derive(Debug)]
struct Globals {
    assets: Assets,
    exec_time: f32,
    map: Map,
    car3: Ai,
    car2: Ai,
    car1: Racer,
    state: State,
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
            state: State::WaitingForInput,
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

    fn set_car1(&mut self, ctx: &Context, car1: Racer) {
        self.car1.remove(&mut self.map);
        let animation = TranslationAnimation::new(
            get_current_time(ctx),
            0.25,
            self.car1.position.to_point(),
            car1.position.to_point(),
            DrawableObject::DrawableCar(car1.to_car()),
        );
        self.state = State::WaitingForAnimation(animation, car1);
    }

    fn turn_left(&mut self, ctx: &Context) {
        let car1 = self.car1.turn_left();
        self.set_car1(ctx, car1)
    }

    fn turn_right(&mut self, ctx: &Context) {
        let car1 = self.car1.turn_right();
        self.set_car1(ctx, car1)
    }

    fn go_forward(&mut self, ctx: &Context) {
        if let Some(car1) = self.car1.go_forward(&self.map) {
            self.set_car1(ctx, car1);
        }
    }

    fn go_backwards(&mut self, ctx: &Context) {
        if let Some(car1) = self.car1.go_backwards(&self.map) {
            self.set_car1(ctx, car1);
        }
    }

    fn go_back_to_checkpoint(&mut self, ctx: &Context) {
        let car1 = self.car1.go_back_to_checkpoint(&self.map);
        self.set_car1(ctx, car1);
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            State::WaitingForAnimation(animation, car1) => {
                let current_time = get_current_time(ctx);
                if animation.is_finished(current_time) {
                    self.car1 = car1;
                    self.car1.insert(&mut self.map);
                    self.state = State::WaitingForInput;
                }
            },
            State::WaitingForInput => (),
        }
        Ok(())
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match self.state {
            State::WaitingForInput => {
                let action3 = self.car3.next_action();
                let car3 = self.car3.perform_action(action3, &mut self.map);
                self.set_car3(car3);

                let action2 = self.car2.next_action();
                let car2 = self.car2.perform_action(action2, &mut self.map);
                self.set_car2(car2);

                match keycode {
                    Keycode::Left  => self.turn_left(ctx),
                    Keycode::Right => self.turn_right(ctx),
                    Keycode::Up    => self.go_forward(ctx),
                    Keycode::Down  => self.go_backwards(ctx),
                    Keycode::R     => self.go_back_to_checkpoint(ctx),
                    _              => (),
                }
            },
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        bg::draw_bg(ctx, &self.assets.bg)?;

        hex::draw_hex_grid(
            ctx,
            &self.assets.hex,
        )?;
        self.map.draw(ctx, &self.assets.map, &self.assets.car)?;

        match self.state {
            State::WaitingForAnimation(animation, _car1) => {
                let current_time = get_current_time(ctx);
                animation.draw(ctx, &self.assets.car, current_time)?;
            },
            State::WaitingForInput => (),
        }

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
