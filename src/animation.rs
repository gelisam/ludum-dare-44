use core::time::Duration;
use ggez::{GameResult, Context};
use ggez::graphics::{Point2,Vector2};

use car::{self,Car};
use globals::*;
use hex::*;


#[derive(Clone, Copy, Debug)]
pub enum DrawableObject {
    DrawableCar(Car),
}

#[derive(Clone, Copy, Debug)]
pub struct TranslationAnimation {
    start_time: Duration,
    duration: f32,
    start_position: Point2,
    delta: Vector2,
    drawable_object: DrawableObject,
}

impl TranslationAnimation {
    pub fn new(
        start_time: Duration,
        duration: f32,
        start_position: Point2,
        end_position: Point2,
        drawable_object: DrawableObject,
    ) -> TranslationAnimation {
        TranslationAnimation {
            start_time,
            duration,
            start_position,
            delta: end_position - start_position,
            drawable_object,
        }
    }

    pub fn new_default() -> TranslationAnimation {
        let default_draw_obj = DrawableObject::DrawableCar(Car::new(0, HexVector::new(0, 0)));
        TranslationAnimation {
            start_time: Duration::new(0, 0),
            duration: 0.0,
            start_position: Point2::new(0.0, 0.0),
            delta: Vector2::new(0.0, 0.0),
            drawable_object: default_draw_obj,
        }
    }

    pub fn is_finished(self, current_time: Duration) -> bool {
        let fraction = duration_to_f32(current_time - self.start_time) / self.duration;
        fraction >= 1.0
    }

    pub fn draw(self, ctx: &mut Context, car_assets: &car::Assets, current_time: Duration) -> GameResult<()> {
        let fraction = duration_to_f32(current_time - self.start_time) / self.duration;
        let current_position = self.start_position + self.delta * fraction;

        match self.drawable_object {
            DrawableObject::DrawableCar(car) =>
                car.draw(ctx, car_assets, current_position),
        }
    }
}
