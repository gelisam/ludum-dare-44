extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font};

mod car;
mod center;
mod globals;
mod hex;
mod map;
mod text;
mod vector;

use globals::*;
use map::Map;


#[derive(Debug)]
struct Assets {
    hex: hex::Assets,
    map: map::Assets,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        hex: hex::load_assets(ctx)?,
        map: map::load_assets(ctx, &font)?,
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
    background: graphics::Image,
    assets: Assets,
    map: Map,
    mouse: Mouse,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        //FIXME: Err "Resource Not Found" in runtime. Now, we temperiarly work
        //          around it by manually putting the `resources/` dir into
        //          `target/debug/`. Fix it when u got the free time and there
        //          are some documents and issues tracker here.
        //
        //          FAQ: https://github.com/ggez/ggez/blob/master/docs/FAQ.md#i-get-resourcenotfoundmyfile--even-though-its-in-the-resource-dir
        let background_img = graphics::Image::new(ctx, "/dragon1.png")?;
        
        Ok(Globals {
            background: background_img,
            assets: load_assets(ctx)?,
            map: Map::load(),
            mouse: Mouse::new()
        })
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.map.go_forward();
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        // Configuration of images
        let background_param = graphics::DrawParam {
            //src:
            dest: graphics::Point2::new(0.0, 0.0),
            //rotation:
            //offset:
            scale: graphics::Point2::new(0.2, 0.2),
            ..Default:: default()
        };

        // Draw it!
        graphics::draw_ex(ctx, &self.background, background_param)?;

        hex::draw_hex_grid(
            ctx,
            &self.assets.hex,
        )?;
        self.map.draw(ctx, &self.assets.map)?;

        graphics::present(ctx);
        Ok(())
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: event::MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        if self.mouse.mouse_down {
            self.mouse.pos_x = x;
            self.mouse.pos_y = y;
        }
        println!(
            "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
            x, y, xrel, yrel
        );
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context,
            button: event::MouseButton, x: i32, y: i32) {
        self.mouse.mouse_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context,
            button: event::MouseButton, x: i32, y: i32) {
        self.mouse.mouse_down = false;
        println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
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
