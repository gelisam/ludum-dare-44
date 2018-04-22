use ggez::{GameResult, Context};
use ggez::graphics::{Image, Point2};

use globals::*;
use center::*;
use hex::HexVector;


#[derive(Debug)]
pub struct Assets {
    car3:            Image,
    car2:            Image,
    car1:            Image,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(
        Assets {
            car3:            Image::new(ctx, "/car3.png")?,
            car2:            Image::new(ctx, "/car2.png")?,
            car1:            Image::new(ctx, "/car1.png")?,
        }
    )
}


// 0..3
pub type CarNumber = usize;

#[derive(Clone, Copy, Debug)]
pub struct Car {
    pub car_number: CarNumber,
    pub direction: HexVector,
}

impl Car {
    pub fn new(car_number: CarNumber, direction: HexVector) -> Car {
        Car {car_number, direction}
    }

    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: Point2) -> GameResult<()> {
        let car_assets = [&assets.car1, &assets.car2, &assets.car3];
        draw_centered(ctx, car_assets[self.car_number - 1], image_size(), dest, self.direction.to_rotation())
    }
}
