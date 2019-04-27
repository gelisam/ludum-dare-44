use core::time::Duration;
use ggez::Context;
use ggez::timer;


pub const GAME_NAME: &str = "ludum-dare-44";

pub const WINDOW_WIDTH:  u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;


#[allow(dead_code)]
pub const PI:     f32 = 3.141592653589793;
pub const SQRT_3: f32 = 1.7320508075688772;


pub fn get_current_time(ctx: &Context) -> Duration {
    timer::get_time_since_start(ctx)
}

pub fn duration_to_f32(duration: Duration) -> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1000000000.0
}
