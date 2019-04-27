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
    border_images: Vec<Image>,
    center_images: Vec<Image>,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        bg: bg::load_assets(ctx)?,
        dot: Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 10.0, 3.0)?,
        font,
        hex: hex::load_assets(ctx)?,
        border_images: vec!(
            Image::new(ctx, "/branch dia 1.png")?,
            Image::new(ctx, "/branch dia 2.png")?,
            Image::new(ctx, "/branch anti-dia 1.png")?,
            Image::new(ctx, "/branch anti-dia 2.png")?,
            Image::new(ctx, "/branch vert 1.png")?,
            Image::new(ctx, "/branch vert 2.png")?,
            Image::new(ctx, "/branch vert 3.png")?,
            Image::new(ctx, "/trunk dia 1.png")?,
            Image::new(ctx, "/trunk dia 2.png")?,
            Image::new(ctx, "/trunk anti-dia 1.png")?,
            Image::new(ctx, "/trunk anti-dia 2.png")?,
            Image::new(ctx, "/trunk vert 1.png")?,
            Image::new(ctx, "/trunk vert 2.png")?,
        ),
        center_images: vec!(
            Image::new(ctx, "/leaves 1.png")?,
            Image::new(ctx, "/leaves 2.png")?,
            Image::new(ctx, "/flower 1.png")?,
            Image::new(ctx, "/flower 2.png")?,
            Image::new(ctx, "/flowers 3.png")?,
            Image::new(ctx, "/beehive.png")?,
        ),
    })
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    start_time: Duration,
    bees: channel::Channel,
    birds: channel::Channel,
    bounty: sidebar::Sidebar,
    life: sidebar::Sidebar,
    hover: Option<hex::HexPoint>,
    branches: HashMap<hex::HexPoint, usize>,
    gifts: HashMap<hex::HexPoint, usize>,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        let assets = load_assets(ctx)?;
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
        branches.insert(hex::HexPoint::new(0, 1), 12);

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
        let hex_point = hex::HexPoint::from_point(Point2::new(x as f32, y as f32));

        match button {
            MouseButton::Left => {
                if hex_point.is_cell_center() {
                    match self.gifts.get(&hex_point).map (|x| *x) {
                        None => {
                            self.gifts.insert(hex_point, 0);
                        },
                        Some(i) => {
                            self.gifts.insert(hex_point, (i + 1) % self.assets.center_images.len());
                        },
                    }
                } else if hex_point.is_cell_border() {
                    match self.branches.get(&hex_point).map (|x| *x) {
                        None => {
                            self.branches.insert(hex_point, 0);
                        },
                        Some(i) => {
                            self.branches.insert(hex_point, (i + 1) % self.assets.border_images.len());
                        },
                    }
                }
            },
            MouseButton::Right => {
                if hex_point.is_cell_center() {
                    self.gifts.remove(&hex_point);
                } else if hex_point.is_cell_border() {
                    self.branches.remove(&hex_point);
                }
            }
            _ => {}
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _state: MouseState, x: i32, y: i32, _xrel: i32, _yrel: i32) {
        let hex_point = hex::HexPoint::from_point(Point2::new(x as f32, y as f32));
        self.hover = if hex_point.is_in_bounds() {
            Some(hex_point)
        } else {
            None
        };
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
            let point = hex_point.to_point();
            let image = &self.assets.border_images[*branch_index];
            center::draw_centered(ctx, image, Vector2::new(image.width() as f32, image.height() as f32), point, 0.0)?;
        }
        for (hex_point, gift_index) in self.gifts.iter() {
            let point = hex_point.to_point();
            let image = &self.assets.center_images[*gift_index];
            center::draw_centered(ctx, image, Vector2::new(image.width() as f32, image.height() as f32), point, 0.0)?;
        }

        if let Some(hex_point) = self.hover {
            set_color(ctx, Color::from_rgb(255, 128, 128))?;
            self.assets.dot.draw(ctx, hex_point.to_point(), 0.0)?;
        }

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
