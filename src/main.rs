extern crate core;
extern crate ggez;
extern crate rand;
extern crate counter;

use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::event::*;
use ggez::graphics::*;
use ggez::timer;
use std::collections::HashMap;

mod bg;
mod cell;
mod center;
mod channel;
mod globals;
mod hex;
mod sidebar;
mod text;
mod vector;
mod life;

use globals::*;

#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    cell: cell::Assets,
    dot: Mesh,
    font: Font,
    hex: hex::Assets,
}

impl Assets {
    fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
        let font = Font::default_font()?;

        Ok(Assets {
            bg: bg::load_assets(ctx)?,
            cell: cell::load_assets(ctx)?,
            dot: Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 10.0, 3.0)?,
            font,
            hex: hex::load_assets(ctx)?,
        })
    }
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    start_time: Duration,
    turn_time: Duration,
    turn_duration: Duration,
    bees: channel::Channel,
    birds: channel::Channel,
    bounty: sidebar::Sidebar,
    life: sidebar::Sidebar,
    bounty_amount: f32,
    life_amount: f32,
    hover: Option<hex::InBoundsPoint>,
    branches: HashMap<hex::BranchPoint, cell::BranchCell>,
    gifts: HashMap<hex::GiftPoint, cell::GiftCell>,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        let assets = Assets::load_assets(ctx)?;
        let bounty = sidebar::Sidebar::new(
            ctx,
            &assets.font,
            "Bounty",
            Color::from_rgb(181, 208, 212),
            0.0
        )?;
        let life = sidebar::Sidebar::new(
            ctx,
            &assets.font,
            "Life",
            Color::from_rgb(242, 240, 186),
            WINDOW_WIDTH as f32 - sidebar::SIDEBAR_WIDTH
        )?;

        let mut branches = HashMap::with_capacity(100);
        let root_point = hex::BranchPoint::new(hex::HexPoint::new(0, 1));
        let root_cell = cell::BranchCell::new(
            &assets.cell,
            &mut rand::thread_rng(),
            None,
            root_point
        );
        branches.insert(root_point, root_cell);

        let mut gifts = HashMap::with_capacity(100);
        let origin_point = hex::GiftPoint::new(hex::HexPoint::new(0, 0));
        let origin_cell = cell::GiftCell::new(root_point);
        gifts.insert(origin_point, origin_cell);

        Ok(Globals {
            assets,
            start_time: get_current_time(ctx),
            turn_time: get_current_time(ctx),
            turn_duration: Duration::from_millis(2000),
            bees: channel::Channel::new(ctx, "/bees.ogg")?,
            birds: channel::Channel::new(ctx, "/birds.ogg")?,
            bounty,
            life,
            bounty_amount: 5.0f32,
            life_amount: 0.0f32,
            hover: None,
            branches,
            gifts,
        })
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.start_time = get_current_time(ctx);
    }
}

impl EventHandler for Globals {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.bees.update(ctx);
        self.birds.update(ctx);
        self.bounty.update(ctx, self.bounty_amount, 0.0f32);
        self.life.update(ctx, 0.0f32, self.life_amount+1.0);

        let now = get_current_time(ctx);
        while (now - self.turn_time) > self.turn_duration { // while loop in case of large discrepancy
            let basic_amount = 0.1f32; // get this amount even if no life
            self.bounty_amount = (self.bounty_amount+self.life_amount+basic_amount).min(30.0);
            self.turn_time = self.turn_time + self.turn_duration;

            life::life_cycle(&mut self.gifts, &self.branches);
        }

        ggez::timer::sleep(Duration::from_millis(50));
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Z      => self.bees.set_future_volume(ctx, Duration::from_millis(1000), 1.0),
            Keycode::X      => self.birds.set_future_volume(ctx, Duration::from_millis(1000), 1.0),
            Keycode::Escape => ctx.quit().unwrap(),
            _               => (),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Z     => self.bees.set_future_volume(ctx, Duration::from_millis(1000), 0.0),
            Keycode::X     => self.birds.set_future_volume(ctx, Duration::from_millis(1000), 0.0),
            Keycode::R     => self.reset(ctx),
            _              => (),
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        let point = Point2::new(x as f32, y as f32);
        if let Some(in_bounds_point) = hex::HexPoint::from_point(point).is_in_bounds() {
            match button {
                MouseButton::Left => {
                    match in_bounds_point {
                        hex::InBoundsPoint::BranchPoint(branch_point) => {
                            let assets_ = &mut self.assets;
                            let bounty_amount_ = &mut self.bounty_amount;
                            let life_amount_ = &mut self.life_amount;
                            let gifts_ = &mut self.gifts;
                            match self.branches.get(&branch_point) {
                                None => {
                                    let gift_neighbours = branch_point.gift_neighbours();
                                    let empty_neighbours: Vec<hex::GiftPoint> = gift_neighbours
                                        .iter()
                                        .map(|g| *g)
                                        .filter(|g| gifts_.get(g).is_none())
                                        .collect();
                                    let full_neighbours: Vec<hex::GiftPoint> = gift_neighbours
                                        .iter()
                                        .map(|g| *g)
                                        .filter(|g| gifts_.get(g).is_some())
                                        .collect();
                                    if empty_neighbours.len() == 1 && full_neighbours.len() == 1 {
                                        let empty_neighbour = empty_neighbours[0];
                                        let full_neighbour = full_neighbours[0];
                                        if *bounty_amount_ >= 1.0 {
                                            // place a new branch
                                            *bounty_amount_ -= 1.0;
                                            *life_amount_ += 0.1;
                                            let branch_cell = cell::BranchCell::new(
                                                &assets_.cell,
                                                &mut rand::thread_rng(),
                                                Some(full_neighbour),
                                                branch_point
                                            );
                                            let gift_cell = cell::GiftCell::new(branch_point);
                                            self.branches.insert(branch_point, branch_cell);
                                            gifts_.insert(empty_neighbour, gift_cell);
                                        } else {
                                            println!("not enough Bounty");
                                        }
                                    } else if empty_neighbours.len() == 2 {
                                        println!("new branches must be attached to the tree");
                                    } else if full_neighbours.len() == 2 {
                                        println!("branches cannot form a cycle");
                                    }
                                },
                                Some(_) => {
                                    if let Some(branch_cell) = self.branches.get_mut(&branch_point) {
                                        match branch_cell.branch_upgrade {
                                            0 => {
                                                // upgrade a branch to level 1
                                                if *bounty_amount_ >= 2.0 {
                                                    *bounty_amount_ -= 2.0;
                                                    *life_amount_ += 0.1;
                                                    branch_cell.upgrade(&assets_.cell, &mut rand::thread_rng(), branch_point, 1);
                                                } else {
                                                    println!("not enough Bounty");
                                                }
                                            },
                                            1 => {
                                                // upgrade a branch to level 2
                                                if *bounty_amount_ >= 3.0 {
                                                    *bounty_amount_ -= 3.0;
                                                    *life_amount_ += 0.1;
                                                    branch_cell.upgrade(&assets_.cell, &mut rand::thread_rng(), branch_point, 2);
                                                } else {
                                                    println!("not enough Bounty");
                                                }
                                            },
                                            2 => {
                                                // upgrade a branch to level 3
                                                if *bounty_amount_ >= 4.0 {
                                                    *bounty_amount_ -= 4.0;
                                                    *life_amount_ += 0.1;
                                                    branch_cell.upgrade(&assets_.cell, &mut rand::thread_rng(), branch_point, 3);
                                                } else {
                                                    println!("not enough Bounty");
                                                }
                                            },
                                            _ => {
                                                println!("this branch has already reached its maximum growth");
                                            },
                                        }
                                    }
                                },
                            }
                        },
                        hex::InBoundsPoint::GiftPoint(gift_point) => {
                            match self.gifts.get(&gift_point) {
                                None => {
                                    println!("you cannot place a branch on a cell, only in-between two cells");
                                },
                                Some(gift_cell) => {
                                    match gift_cell.gift {
                                        None => {
                                            println!("you cannot place leaves, you have to let them grow");
                                        },
                                        Some(cell::Gift::Leaves) => {
                                            println!("right-click to release the leaf");
                                        },
                                        Some(cell::Gift::Flowers) => {
                                            println!("right-click to release the flower");
                                        },
                                        Some(cell::Gift::Berries) => {
                                            println!("right-click to release the berry");
                                        },
                                        Some(cell::Gift::Nuts) => {
                                            println!("right-click to release the nut");
                                        },
                                        Some(cell::Gift::Beehive) => {
                                            println!("right-click to release the beehive");
                                        },
                                        Some(cell::Gift::Birdnest) => {
                                            println!("right-click to release the bird nest");
                                        },
                                    }
                                },
                            }
                        },
                    }
                },
                MouseButton::Right => {
                    match in_bounds_point {
                        hex::InBoundsPoint::BranchPoint(branch_point) => {
                            self.branches.remove(&branch_point);
                        },
                        hex::InBoundsPoint::GiftPoint(gift_point) => {
                            self.gifts
                                .entry(gift_point)
                                .and_modify(|g| g.gift = None);
                        },
                    }
                }
                _ => {}
            }
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState, x: i32, y: i32, _xrel: i32, _yrel: i32) {
        let hex_point = hex::HexPoint::from_point(Point2::new(x as f32, y as f32));
        self.hover = hex_point.is_in_bounds()
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // must use white for drawing images, otherwise they get tinted
        set_color(ctx, Color::from_rgb(255, 255, 255))?;

        bg::draw_bg(ctx, &self.assets.bg)?;
        hex::draw_hex_grid(ctx, &self.assets.hex)?;
        self.bounty.draw(ctx)?;
        self.life.draw(ctx)?;

        set_color(ctx, Color::from_rgb(255, 255, 255))?;
        for (&branch_point, branch_cell) in self.branches.iter() {
            branch_cell.draw(ctx, &self.assets.cell, branch_point)?;
        }
        for (&gift_point, gift_cell) in self.gifts.iter() {
            gift_cell.draw(ctx, &self.assets.cell, gift_point)?;
        }

        if let Some(in_bounds_point) = self.hover {
            set_color(ctx, Color::from_rgb(255, 128, 128))?;
            self.assets.dot.draw(ctx, in_bounds_point.to_point(), 0.0)?;

            // neighbour-debugging code; uncomment me, it's fun!
            //set_color(ctx, Color::from_rgb(128, 128, 255))?;
            //match in_bounds_point {
            //    hex::InBoundsPoint::BranchPoint(branch_point) => {
            //        for n in branch_point.gift_neighbours() {
            //            self.assets.dot.draw(ctx, n.to_point(), 0.0)?;
            //        }
            //    },
            //    hex::InBoundsPoint::GiftPoint(gift_point) => {
            //        for n in gift_point.branch_neighbours() {
            //            self.assets.dot.draw(ctx, n.to_point(), 0.0)?;
            //        }
            //        for n in gift_point.gift_neighbours() {
            //            self.assets.dot.draw(ctx, n.to_point(), 0.0)?;
            //        }
            //    },
            //}
        }

        //println!("FPS: {}", ggez::timer::get_fps(ctx));
        present(ctx);
        timer::yield_now();

        Ok(())
    }
}

pub fn main() {
    let ctx = &mut Context::load_from_conf(
        GAME_NAME,
        "Michaelson Britt, Samuel Gélineau, Zhentao Li, Kyla Squires, and Farren Wang",
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
    globals.bees.source.play().unwrap();
    globals.birds.source.play().unwrap();

    run(ctx, globals).unwrap();
}
