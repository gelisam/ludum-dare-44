extern crate core;
extern crate ggez;
extern crate rand;

use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
use ggez::timer;
use rand::Rng;

mod ai;
mod animation;
mod bg;
mod bomb;
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
use hex::{HexPoint};
use map::Map;
use racer::Racer;

#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    bomb: bomb::Assets,
    car: car::Assets,
    hex: hex::Assets,
    map: map::Assets,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(Assets {
        bg: bg::load_assets(ctx)?,
        bomb: bomb::load_assets(ctx)?,
        car: car::load_assets(ctx)?,
        hex: hex::load_assets(ctx)?,
        map: map::load_assets(ctx)?,
    })
}


#[derive(Clone, Copy, Debug)]
enum State {
    WaitingForInput,
    WaitingForAnimation(TranslationAnimation, Racer,
        TranslationAnimation, Ai, TranslationAnimation, Ai),
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
        let (exec_time, map, car3, car2, car1, state) = Globals::new_everything();

        Ok(Globals {
            assets: load_assets(ctx)?,
            exec_time,
            map,
            car3,
            car2,
            car1,
            state,
        })
    }

    fn reset(&mut self) {
        let (exec_time, map, car3, car2, car1, state) = Globals::new_everything();

        self.exec_time = exec_time;
        self.map       = map;
        self.car3      = car3;
        self.car2      = car2;
        self.car1      = car1;
        self.state     = state;
    }

    fn new_everything() -> (f32, Map, Ai, Ai, Racer, State) {
        let mut map = Map::load();

        let car3 = Racer::new(3, HexPoint::new(CENTRAL_OBSTACLE_RADIUS+1, 0));
        car3.insert(&mut map);

        let car2 = Racer::new(2, HexPoint::new(CENTRAL_OBSTACLE_RADIUS+2, 0));
        car2.insert(&mut map);

        let car1 = Racer::new(1, HexPoint::new(CENTRAL_OBSTACLE_RADIUS+3, 0));
        car1.insert(&mut map);

        let mut rng = rand::thread_rng();
        for _i in 0..15 {
            if let Some(hex_point) = map.random_available_spot() {
                let fuse_length = rng.gen_range(1, MAX_FUSE_LENGTH+1);
                map.insert_bomb(hex_point, fuse_length);
            }
        }

        (0.0, map, Ai::new(car3), Ai::new(car2), car1, State::WaitingForInput)
    }

    fn set_car3(&mut self, ctx: &Context, car3: Ai) {
        self.car3.racer.remove(&mut self.map);
        let animation = TranslationAnimation::new(
            get_current_time(ctx),
            0.25,
            self.car3.racer.position.to_point(),
            car3.racer.position.to_point(),
            DrawableObject::DrawableCar(car3.racer.to_car()),
        );
        match self.state {
            State::WaitingForAnimation(a1, c1, a2, c2, _a3, _c3) =>
                self.state = State::WaitingForAnimation(a1, c1, a2, c2, animation, car3),
            _ => {
                let a1 = TranslationAnimation::new_default();
                let a2 = TranslationAnimation::new_default();
                let c1 = Racer::new(0, HexPoint::new(0,0));
                let c2 = Ai::new(Racer::new(0, HexPoint::new(0,0)));
                self.state = State::WaitingForAnimation(a1, c1, a2, c2,
                    animation, car3)
            },
        }
        // self.state = State::WaitingForAnimation(animation, car3: car3.racer);
        // self.car3 = car3;
        // self.car3.racer.insert(&mut self.map);
    }

    fn set_car2(&mut self, ctx: &Context, car2: Ai) {
        self.car2.racer.remove(&mut self.map);
        let animation = TranslationAnimation::new(
            get_current_time(ctx),
            0.25,
            self.car2.racer.position.to_point(),
            car2.racer.position.to_point(),
            DrawableObject::DrawableCar(car2.racer.to_car()),
        );

        match self.state {
            State::WaitingForAnimation(a1, c1, _a2, _c2, a3, c3) => 
                self.state = State::WaitingForAnimation(a1, c1, animation, car2,
                    a3, c3),
            _ => {
                let a1 = TranslationAnimation::new_default();
                let a3 = TranslationAnimation::new_default();
                let c1 = Racer::new(0, HexPoint::new(0,0));
                let c3 = Ai::new(Racer::new(0, HexPoint::new(0,0)));
                self.state = State::WaitingForAnimation(a1, c1, animation,
                    car2, a3, c3)},
        }
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
        match self.state {
            State::WaitingForAnimation(_a1, _c1, a2, c2, a3, c3) =>
                self.state = State::WaitingForAnimation(animation, car1, a2, c2,
                    a3, c3),
            _ => {
                let a2 = TranslationAnimation::new_default();
                let a3 = TranslationAnimation::new_default();
                let c2 = Ai::new(Racer::new(0, HexPoint::new(0,0)));
                let c3 = Ai::new(Racer::new(0, HexPoint::new(0,0)));
                self.state = State::WaitingForAnimation(animation, car1, a2, c2,
                    a3, c3)
            },
        }
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
            State::WaitingForAnimation(animation1, car1, animation2, car2,
                animation3, car3) => {
                let current_time = get_current_time(ctx);
                if animation1.is_finished(current_time) {
                    self.car1 = car1;
                    self.car1.insert(&mut self.map);
                }
                if animation2.is_finished(current_time) {
                    self.car2 = car2;
                    self.car2.racer.insert(&mut self.map);
                }

                if animation3.is_finished(current_time) {
                    self.car3 = car3;
                    self.car3.racer.insert(&mut self.map);
                }
                
                if animation1.is_finished(current_time)
                    && animation2.is_finished(current_time)
                    && animation3.is_finished(current_time) {
                        self.state = State::WaitingForInput;
                        self.map.decrement_all_bombs();
                        self.state = State::WaitingForInput;
                        } else {
                        self.state = self.state;
                    };
                                    
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
                self.set_car3(ctx, car3);

                let action2 = self.car2.next_action();
                let car2 = self.car2.perform_action(action2, &mut self.map);
                self.set_car2(ctx, car2);

                match keycode {
                    Keycode::Left  => self.turn_left(ctx),
                    Keycode::Right => self.turn_right(ctx),
                    Keycode::Up    => self.go_forward(ctx),
                    Keycode::Down  => self.go_backwards(ctx),
                    Keycode::K     => self.go_back_to_checkpoint(ctx),
                    Keycode::R     => self.reset(),
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
        self.map.draw(ctx, &self.assets.map, &self.assets.bomb, &self.assets.car)?;

        match self.state {
            State::WaitingForAnimation(animation1, _car1, animation2, _car2,
                animation3, _car3) => {
                let current_time = get_current_time(ctx);
                animation1.draw(ctx, &self.assets.car, current_time)?;
                animation2.draw(ctx, &self.assets.car, current_time)?;
                animation3.draw(ctx, &self.assets.car, current_time)?;
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
