extern crate core;
extern crate ggez;
extern crate rand;

use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::event::{self, Keycode, Mod};
use ggez::graphics::{self, Font};
use ggez::timer;
use ggez::audio;

mod bg;
mod center;
mod globals;
mod text;
mod vector;

use globals::*;


#[derive(Debug)]
struct Assets {
    bg: bg::Assets,
    font: Font,
}

fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    let font = Font::default_font()?;

    Ok(Assets {
        bg: bg::load_assets(ctx)?,
        font,
    })
}

#[derive(Debug)]
struct Channel {
    source: audio::Source,
    start_time: Duration,
    duration: Duration,
    initial_volume: f32,
    target_volume: f32,
}

impl Channel {
    fn new(ctx: &mut Context, path: &'static str) -> GameResult<Channel> {
        let mut source = audio::Source::new(ctx, path)?;
        source.set_repeat(true);
        source.set_volume(0.0);

        Ok(Channel {
            source,
            start_time: get_current_time(ctx),
            duration: timer::f64_to_duration(0.0),
            initial_volume: 0.0,
            target_volume: 0.0,
        })
    }

    fn set_future_volume(&mut self, ctx: &mut Context, duration: Duration, volume: f32) {
        self.start_time = get_current_time(ctx);
        self.duration = duration;
        self.initial_volume = self.source.volume();
        self.target_volume = volume;
    }

    fn update(&mut self, ctx: &mut Context) {
        let t0 = timer::duration_to_f64(self.start_time) as f32;
        let t1 = timer::duration_to_f64(self.start_time + self.duration) as f32;
        let dt = timer::duration_to_f64(self.duration) as f32;
        let t = timer::duration_to_f64(get_current_time(ctx)) as f32;
        let v0 = self.initial_volume;
        let v1 = self.target_volume;
        let dv = v1 - v0;
        if t >= t1 {
            self.source.set_volume(v1);
        } else {
            let v = v0 + (t - t0) * dv / dt;
            self.source.set_volume(v);
        }
    }
}

#[derive(Debug)]
struct Globals {
    assets: Assets,
    start_time: Duration,
    bees: Channel,
    birds: Channel,
}

impl Globals {
    fn new(ctx: &mut Context) -> GameResult<Globals> {
        Ok(Globals {
            assets: load_assets(ctx)?,
            start_time: get_current_time(ctx),
            bees: Channel::new(ctx, "/bees.ogg")?,
            birds: Channel::new(ctx, "/birds.ogg")?,
        })
    }

    fn reset(&mut self, ctx: &mut Context) {
        self.start_time = get_current_time(ctx);
    }
}

impl event::EventHandler for Globals {
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

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        bg::draw_bg(ctx, &self.assets.bg)?;

        let msg = format!("{:#?}", get_current_time(ctx) - self.start_time);
        let text = graphics::Text::new(
            ctx,
            &msg,
            &self.assets.font
        )?;
        let center = graphics::Point2::new(
            WINDOW_WIDTH as f32 / 2.0,
            WINDOW_HEIGHT as f32 / 2.0,
        );
        text::draw_centered_text(ctx, &text, center, 0.0)?;

        graphics::present(ctx);
        timer::yield_now();

        Ok(())
    }
}

pub fn main() {
    let ctx = &mut Context::load_from_conf(
        GAME_NAME,
        "Michaelson Britt and Samuel Gélineau",
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

    event::run(ctx, globals).unwrap();
}
