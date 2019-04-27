use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::timer;
use ggez::audio;

use globals::*;


#[derive(Debug)]
pub struct Channel {
    pub source: audio::Source,
    pub start_time: Duration,
    pub duration: Duration,
    pub initial_volume: f32,
    pub target_volume: f32,
}

impl Channel {
    pub fn new(ctx: &mut Context, path: &'static str) -> GameResult<Channel> {
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

    pub fn set_future_volume(&mut self, ctx: &mut Context, duration: Duration, volume: f32) {
        self.start_time = get_current_time(ctx);
        self.duration = duration;
        self.initial_volume = self.source.volume();
        self.target_volume = volume;
    }

    pub fn update(&mut self, ctx: &mut Context) {
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
