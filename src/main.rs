extern crate core;
extern crate ggez;
extern crate rand;

use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::event::*;
use ggez::graphics::*;
use ggez::timer;
use std::collections::HashMap;

mod bg;
mod center;
mod channel;
mod globals;
mod hex;
mod sidebar;
mod text;
mod vector;

use globals::*;


#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    dot: Mesh,
    font: Font,
    hex: hex::Assets,
    vert_branch_images: Vec<Image>,
    diag_branch_images: Vec<Image>,
    anti_diag_branch_images: Vec<Image>,
    gift_images: Vec<Image>,
}

impl Assets {
    fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
        let font = Font::default_font()?;

        Ok(Assets {
            bg: bg::load_assets(ctx)?,
            dot: Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 10.0, 3.0)?,
            font,
            hex: hex::load_assets(ctx)?,
            vert_branch_images: vec!(
                Image::new(ctx, "/twig vert 1.png")?,
                Image::new(ctx, "/twig vert 2.png")?,
                Image::new(ctx, "/twig vert 3.png")?,
                Image::new(ctx, "/branch vert 1.png")?,
                Image::new(ctx, "/branch vert 2.png")?,
                Image::new(ctx, "/branch vert 3.png")?,
                Image::new(ctx, "/trunk vert 1.png")?,
                Image::new(ctx, "/trunk vert 2.png")?,
            ),
            diag_branch_images: vec!(
                Image::new(ctx, "/twig dia 1.png")?,
                Image::new(ctx, "/twig dia 2.png")?,
                Image::new(ctx, "/twig dia 3.png")?,
                Image::new(ctx, "/branch dia 1.png")?,
                Image::new(ctx, "/branch dia 2.png")?,
                Image::new(ctx, "/trunk dia 1.png")?,
                Image::new(ctx, "/trunk dia 2.png")?,
            ),
            anti_diag_branch_images: vec!(
                Image::new(ctx, "/twig anti-dia 1.png")?,
                Image::new(ctx, "/twig anti-dia 2.png")?,
                Image::new(ctx, "/twig anti-dia 3.png")?,
                Image::new(ctx, "/branch anti-dia 1.png")?,
                Image::new(ctx, "/branch anti-dia 2.png")?,
                Image::new(ctx, "/trunk anti-dia 1.png")?,
                Image::new(ctx, "/trunk anti-dia 2.png")?,
            ),
            gift_images: vec!(
                Image::new(ctx, "/leaves 1.png")?,
                Image::new(ctx, "/leaves 2.png")?,
                Image::new(ctx, "/flower 1.png")?,
                Image::new(ctx, "/flower 2.png")?,
                Image::new(ctx, "/flowers 3.png")?,
                Image::new(ctx, "/beehive.png")?,
            ),
        })
    }

    fn branch_images(&self, orientation: hex::Orientation) -> &Vec<Image> {
        match orientation {
            hex::Orientation::Vert     => &self.vert_branch_images,
            hex::Orientation::Diag     => &self.diag_branch_images,
            hex::Orientation::AntiDiag => &self.anti_diag_branch_images,
        }
    }
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    start_time: Duration,
    bees: channel::Channel,
    birds: channel::Channel,
    bounty: sidebar::Sidebar,
    life: sidebar::Sidebar,
    hover: Option<hex::InBoundsPoint>,
    branches: HashMap<hex::BranchPoint, usize>,
    gifts: HashMap<hex::GiftPoint, usize>,
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
        branches.insert(hex::BranchPoint::new(hex::HexPoint::new(0, 1)), 7);

        Ok(Globals {
            assets,
            start_time: get_current_time(ctx),
            bees: channel::Channel::new(ctx, "/bees.ogg")?,
            birds: channel::Channel::new(ctx, "/birds.ogg")?,
            bounty,
            life,
            hover: None,
            branches,
            gifts: HashMap::with_capacity(100),
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
                            match self.branches.get(&branch_point).map (|x| *x) {
                                None => {
                                    self.branches.insert(branch_point, 0);
                                },
                                Some(i) => {
                                    self.branches.insert(branch_point, (i + 1) % self.assets.branch_images(branch_point.orientation()).len());
                                },
                            }
                        },
                        hex::InBoundsPoint::GiftPoint(gift_point) => {
                            match self.gifts.get(&gift_point).map (|x| *x) {
                                None => {
                                    self.gifts.insert(gift_point, 0);
                                },
                                Some(i) => {
                                    self.gifts.insert(gift_point, (i + 1) % self.assets.gift_images.len());
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
                            self.gifts.remove(&gift_point);
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
        for (hex_point, branch_index) in self.branches.iter() {
            let image = &self.assets.branch_images(hex_point.orientation())[*branch_index];
            center::draw_centered_image(ctx, image, hex_point.to_point(), 0.0)?;
        }
        for (hex_point, gift_index) in self.gifts.iter() {
            let image = &self.assets.gift_images[*gift_index];
            center::draw_centered_image(ctx, image, hex_point.to_point(), 0.0)?;
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
