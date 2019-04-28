extern crate core;
extern crate ggez;
extern crate rand;
extern crate counter;

use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::audio;
use ggez::event::*;
use ggez::graphics::*;
use ggez::timer;
use rand::seq::SliceRandom;
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
    branch_place_sound: audio::Source,
    branch_break_sounds: Vec<audio::Source>,
    moss: Image,
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
            branch_place_sound: audio::Source::new(ctx, "/branch_place.ogg")?,
            branch_break_sounds: vec!(
                audio::Source::new(ctx, "/branch_break.ogg")?,
                audio::Source::new(ctx, "/branch_break2.ogg")?,
                audio::Source::new(ctx, "/branch_break3.ogg")?,
                audio::Source::new(ctx, "/branch_break4.ogg")?,
            ),
            moss: Image::new(ctx, "/moss.png")?,
        })
    }
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    start_time: Duration,
    turn_time: Duration,
    turn_duration: Duration,
    guitar_channel: channel::Channel,
    clarinet_channel: channel::Channel,
    high_pithed_clarinet_channel: channel::Channel,
    dreamy_bells_channel: channel::Channel,
    bounty: sidebar::Sidebar,
    life: sidebar::Sidebar,
    bounty_amount: f32,
    life_amount: f32,
    hover: Option<hex::InBoundsPoint>,
    branches: HashMap<hex::BranchPoint, cell::BranchCell>,
    gifts: HashMap<hex::GiftPoint, cell::GiftCell>,
    forbidden: HashMap<hex::GiftPoint, bool>,
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

        let mut globals = Globals {
            assets,
            start_time: get_current_time(ctx),
            turn_time: get_current_time(ctx),
            turn_duration: Duration::from_millis(2000),
            guitar_channel: channel::Channel::new(ctx, "/guitar.ogg")?,
            clarinet_channel: channel::Channel::new(ctx, "/clarinet.ogg")?,
            high_pithed_clarinet_channel: channel::Channel::new(ctx, "/high-pitched clarinet.ogg")?,
            dreamy_bells_channel: channel::Channel::new(ctx, "/dreamy-bells.ogg")?,
            bounty,
            life,
            bounty_amount: 0.0,
            life_amount: 0.0,
            hover: None,
            branches: HashMap::with_capacity(100),
            gifts: HashMap::with_capacity(100),
            forbidden: HashMap::with_capacity(100),
        };
        globals.reset(ctx);
        Ok(globals)
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.start_time = get_current_time(ctx);
        self.turn_time = get_current_time(ctx);
        self.bounty_amount = 5.0;
        self.life_amount = 0.0;

        self.branches.clear();
        let root_point = hex::BranchPoint::new(hex::HexPoint::new(0, 1));
        let root_gift_point = hex::GiftPoint::new(hex::HexPoint::new(0, 0));
        let mut root_cell = cell::BranchCell::new(None);
        root_cell.branch_upgrade = 3;
        self.branches.insert(root_point, root_cell);
        self.forbidden.insert(root_gift_point, true);

        self.gifts.clear();
        let origin_point = hex::GiftPoint::new(hex::HexPoint::new(0, 0));
        let origin_cell = cell::GiftCell::new(root_point);
        self.gifts.insert(origin_point, origin_cell);
    }

    fn branch_children(&self, branch_point: hex::BranchPoint) -> Vec<hex::GiftPoint> {
        branch_point.gift_neighbours()
            .iter()
            .map(|g| *g)
            .filter(|g|
                match self.gifts.get(g) {
                    None => false,
                    Some(gift_cell) => gift_cell.parent == branch_point,
                }
            )
            .collect()
    }

    fn gift_children(&self, gift_point: hex::GiftPoint) -> Vec<hex::BranchPoint> {
        gift_point.branch_neighbours()
            .iter()
            .map(|b| *b)
            .filter(|b|
                match self.branches.get(b) {
                    None => false,
                    Some(branch_cell) => branch_cell.parent == Some(gift_point),
                }
            )
            .collect()
    }

    fn prune_branch(&mut self, branch_point: hex::BranchPoint) {
        if let Some(_) = self.branches.get(&branch_point) {
            for gift_point in self.branch_children(branch_point) {
                self.prune_gift(gift_point);
            }
            self.branches.remove(&branch_point);
        }
    }

    fn prune_gift(&mut self, gift_point: hex::GiftPoint) {
        if let Some(_) = self.gifts.get(&gift_point) {
            for branch_point in self.gift_children(gift_point) {
                self.prune_branch(branch_point);
            }
            self.gifts.remove(&gift_point);
        }
    }
}

impl EventHandler for Globals {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.guitar_channel.update(ctx);
        self.clarinet_channel.update(ctx);
        self.high_pithed_clarinet_channel.update(ctx);
        self.dreamy_bells_channel.update(ctx);
        self.bounty.update(ctx, self.bounty_amount, 0.0f32);
        self.life.update(ctx, 0.0f32, self.life_amount+1.0);

        let now = get_current_time(ctx);
        while (now - self.turn_time) > self.turn_duration { // while loop in case of large discrepancy
            // let basic_amount = 0.1f32; // get this amount even if no life
            // self.bounty_amount = (self.bounty_amount+self.life_amount+basic_amount).min(30.0);
            self.life_amount = life::life_production(&self.gifts);
            self.bounty_amount = (self.bounty_amount + self.life_amount).min(30.0);
            self.turn_time = self.turn_time + self.turn_duration;

            life::life_cycle(&mut self.gifts, &self.branches, &self.forbidden);
        }

        ggez::timer::sleep(Duration::from_millis(50));
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Z      => self.guitar_channel.set_future_volume(ctx, Duration::from_millis(1000), 1.0),
            Keycode::X      => self.clarinet_channel.set_future_volume(ctx, Duration::from_millis(1000), 1.0),
            Keycode::C      => self.high_pithed_clarinet_channel.set_future_volume(ctx, Duration::from_millis(1000), 1.0),
            Keycode::V      => self.dreamy_bells_channel.set_future_volume(ctx, Duration::from_millis(1000), 1.0),
            Keycode::Escape => ctx.quit().unwrap(),
            _               => (),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::Z      => self.guitar_channel.set_future_volume(ctx, Duration::from_millis(1000), 0.0),
            Keycode::X      => self.clarinet_channel.set_future_volume(ctx, Duration::from_millis(1000), 0.0),
            Keycode::C      => self.high_pithed_clarinet_channel.set_future_volume(ctx, Duration::from_millis(1000), 0.0),
            Keycode::V      => self.dreamy_bells_channel.set_future_volume(ctx, Duration::from_millis(1000), 0.0),
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
                                        let full_gift_point = full_neighbours[0];
                                        match gifts_.get(&full_gift_point).unwrap().gift {
                                            None => {
                                                let cost = life::BASE * 5.0;
                                                if *bounty_amount_ >= cost {
                                                    // place a new branch
                                                    self.assets.branch_place_sound.play().unwrap_or(());
                                                    *bounty_amount_ -= cost;
                                                    //*life_amount_ += 0.1;
                                                    let branch_cell = cell::BranchCell::new(Some(full_gift_point));
                                                    let gift_cell = cell::GiftCell::new(branch_point);
                                                    self.branches.insert(branch_point, branch_cell);
                                                    gifts_.insert(empty_neighbour, gift_cell);
                                                    self.forbidden.insert(full_gift_point, true);
                                                } else {
                                                    println!("not enough Bounty");
                                                }
                                            },
                                            Some(gift) => {
                                                println!("release the {:} before attaching a new branch", gift.singular());
                                            },
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
                                                let cost = life::BASE * 25.0;
                                                if *bounty_amount_ >= cost {
                                                    *bounty_amount_ -= cost;
                                                    //*life_amount_ += 0.1;
                                                    branch_cell.branch_upgrade = 1;
                                                } else {
                                                    println!("not enough Bounty");
                                                }
                                            },
                                            1 => {
                                                // upgrade a branch to level 2
                                                let cost = life::BASE * 125.0;
                                                if *bounty_amount_ >= cost {
                                                    *bounty_amount_ -= cost;
                                                    //*life_amount_ += 0.1;
                                                    branch_cell.branch_upgrade = 2;
                                                } else {
                                                    println!("not enough Bounty");
                                                }
                                            },
                                            2 => {
                                                // upgrade a branch to level 3
                                                let cost = life::BASE * 625.0;
                                                if *bounty_amount_ >= cost {
                                                    *bounty_amount_ -= cost;
                                                    //*life_amount_ += 0.1;
                                                    branch_cell.branch_upgrade = 3;
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
                                        Some(gift) => {
                                            println!("right-click to release the {:}", gift.singular());
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
                            if self.branches.get(&branch_point).is_some() {
                                self.assets.branch_break_sounds.choose(&mut rand::thread_rng()).unwrap().play().unwrap_or(());
                                self.prune_branch(branch_point);
                            }
                        },
                        hex::InBoundsPoint::GiftPoint(gift_point) => {
                            self.gifts
                                .entry(gift_point)
                                .and_modify(|g| g.gift = None);
                            self.forbidden
                                .entry(gift_point)
                                .and_modify(|b| *b ^= true)
                                .or_insert(true);
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

        set_color(ctx, Color::from_rgb(0, 0, 0))?; // fix white artifacts around the branches
        for (&branch_point, branch_cell) in self.branches.iter() {
            branch_cell.draw(ctx, &self.assets.cell, branch_point)?;
        }
        set_color(ctx, Color::from_rgb(255, 255, 255))?;
        for (&gift_point, gift_cell) in self.gifts.iter() {
            gift_cell.draw(ctx, &self.assets.cell, gift_point)?;
        }
        set_color(ctx, Color::from_rgb(128, 255, 128))?;
        // Need to skip non-tips. Check that children is [] when we get those!
        for (&gift_point, &b) in self.forbidden.iter() {
            //println!("{:?}", self.gift_children(gift_point).len());
            if b && self.gift_children(gift_point).len() == 0 {
                let image = &self.assets.moss;
                center::draw_centered_image(ctx, image, gift_point.to_point(), 0.0)?;
                //self.assets.dot.draw(ctx, gift_point.to_point(), 0.0)?;
            }
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

        //if get_current_time(ctx) - self.start_time > Duration::from_millis(1000) {
        //    self.start_time = get_current_time(ctx);
        //    println!("FPS: {}", ggez::timer::get_fps(ctx));
        //}
        present(ctx);
        timer::yield_now();

        Ok(())
    }
}

pub fn main() {
    let ctx = &mut Context::load_from_conf(
        GAME_NAME,
        "Michaelson Britt, Samuel Gélineau, Dylan Khor, Zhentao Li, Kyla Squires, and Farren Wang",
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
    globals.guitar_channel.source.play().unwrap_or(());
    globals.clarinet_channel.source.play().unwrap_or(());
    globals.high_pithed_clarinet_channel.source.play().unwrap_or(());
    globals.guitar_channel.source.play().unwrap_or(());

    run(ctx, globals).unwrap();
}
