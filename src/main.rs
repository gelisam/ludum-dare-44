extern crate core;
extern crate ggez;

use ggez::{GameResult, Context};
use ggez::event;
use ggez::graphics::{self, Drawable, Point2, Text, Vector2};


const GAME_NAME: &str = "ludum-dare-41";

const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;


fn text_size(text: &Text) -> Vector2 {
    Vector2::new(text.width() as f32, text.height() as f32)
}

fn draw_centered_text(ctx: &mut Context, text: &Text, o: Point2) -> GameResult<()> {
    text.draw(ctx, o - text_size(text) / 2.0, 0.0)?;
    Ok(())
}


struct Assets {
    hello_world: Text,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = graphics::Font::default_font()?;

    Ok(Assets {
        hello_world: Text::new(ctx, "Hello, world!", &font)?,
    })
}

struct Globals {
    assets: Assets,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        Ok(Globals {
            assets: load_assets(ctx)?,
        })
    }
}

impl event::EventHandler for Globals {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        draw_centered_text(
            ctx,
            &self.assets.hello_world,
            Point2::new(
                WINDOW_WIDTH  as f32 / 2.0,
                WINDOW_HEIGHT as f32 / 2.0,
            ),
        )?;

        graphics::present(ctx);
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
