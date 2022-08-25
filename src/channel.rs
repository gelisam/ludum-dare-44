use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::audio;
use ggez::audio::SoundSource;

use globals::*;


#[derive(Debug)]
pub struct Channel {
    pub source: audio::Source,
    pub start_time: Duration,
    pub duration: Duration,
    pub initial_volume: f32,
    pub target_volume: f32,
    playing: bool,
}

impl Channel {
    pub fn new(ctx: &mut Context, path: &'static str) -> GameResult<Channel> {
        let mut source = audio::Source::new(ctx, path)?;
        source.set_repeat(true);
        source.set_volume(0.0);

        Ok(Channel {
            source,
            start_time: get_current_time(ctx),
            duration: Duration::ZERO,
            initial_volume: 0.0,
            target_volume: 0.0,
            playing: false,
        })
    }

    pub fn enable(&mut self, ctx: &mut Context, should_be_playing: bool) {
        if self.playing && !should_be_playing {
            self.playing = false;
            self.set_future_volume(ctx, Duration::from_millis(1000), 0.0);
        } else if !self.playing && should_be_playing {
            self.playing = true;
            self.set_future_volume(ctx, Duration::from_millis(1000), 1.0);
        }
    }

    pub fn set_future_volume(&mut self, ctx: &mut Context, duration: Duration, volume: f32) {
        self.start_time = get_current_time(ctx);
        self.duration = duration;
        self.initial_volume = self.source.volume();
        self.target_volume = volume;
    }

    pub fn update(&mut self, ctx: &mut Context) {
        let t0 = duration_to_f32(self.start_time);
        let t1 = duration_to_f32(self.start_time + self.duration);
        let dt = duration_to_f32(self.duration);
        let t = duration_to_f32(get_current_time(ctx));
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
