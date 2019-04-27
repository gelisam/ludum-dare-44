extern crate nalgebra;

use core::ops::{Add,AddAssign,Mul,MulAssign};
use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, DrawMode, Point2, Mesh, Vector2};

use globals::*;


pub const HEX_RADIUS:   f32 = 32.0;
pub const HEX_WIDTH:    f32 = HEX_RADIUS * SQRT_3;
pub const HEX_HEIGHT:   f32 = HEX_RADIUS * 2.0;


#[derive(Debug)]
pub struct Assets {
    outline: Mesh,
    filled: Mesh,
}

fn load_polygon_asset(ctx: &mut Context, mode: DrawMode) -> GameResult<Mesh> {
    Mesh::new_polygon(
        ctx,
        mode,
        &[
            Point2::new(HEX_WIDTH / 2.0, 0.5 * HEX_RADIUS),
            Point2::new(0.0, HEX_HEIGHT / 2.0),
            Point2::new(-HEX_WIDTH / 2.0, 0.5 * HEX_RADIUS),
            Point2::new(-HEX_WIDTH / 2.0, -0.5 * HEX_RADIUS),
            Point2::new(0.0, -HEX_HEIGHT / 2.0),
            Point2::new(HEX_WIDTH / 2.0, -0.5 * HEX_RADIUS),
        ],
    )
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(Assets {
        outline: load_polygon_asset(ctx, DrawMode::Line(1.0))?,
        filled:  load_polygon_asset(ctx, DrawMode::Fill)?,
    })
}

pub fn draw_hex_grid(ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    let w = (WINDOW_WIDTH as f32 / HEX_WIDTH) as i32 - 1;
    let h = (WINDOW_HEIGHT as f32 / (HEX_RADIUS * 1.5)) as i32 - 1;
    for r in 1..=h {
        for q in 1-(r/2)..=w-(r/2) {
            let hex_point = HexPoint::new(q, r);
            assets.outline.draw(ctx, hex_point.to_point(), 0.0)?;
        }
    }

    Ok(())
}


// using "pointy-topped axial coordinates":
// https://www.redblobgames.com/grids/hexagons/#coordinates-axial
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HexPoint {
    pub q: i32,
    pub r: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct HexVector {
    pub q: i32,
    pub r: i32,
}


impl Add<HexVector> for HexPoint {
    type Output = HexPoint;

    fn add(self, other: HexVector) -> HexPoint {
        HexPoint::new(
            self.q + other.q,
            self.r + other.r,
        )
    }
}

impl AddAssign<HexVector> for HexPoint {
    fn add_assign(&mut self, other: HexVector) {
        self.q += other.q;
        self.r += other.r;
    }
}

impl Add for HexVector {
    type Output = HexVector;

    fn add(self, other: HexVector) -> HexVector {
        HexVector::new(
            self.q + other.q,
            self.r + other.r,
        )
    }
}

impl AddAssign for HexVector {
    fn add_assign(&mut self, other: HexVector) {
        self.q += other.q;
        self.r += other.r;
    }
}

impl Mul<i32> for HexVector {
    type Output = HexVector;

    fn mul(self, factor: i32) -> HexVector {
        HexVector::new(
            self.q * factor,
            self.r * factor,
        )
    }
}

impl MulAssign<i32> for HexVector {
    fn mul_assign(&mut self, factor: i32) {
        self.q *= factor;
        self.r *= factor;
    }
}


impl HexPoint {
    pub fn new(q: i32, r: i32) -> HexPoint {
        HexPoint {q, r}
    }

    #[allow(dead_code)]
    pub fn neighbours(self) -> Vec<HexPoint> {
        (0..6)
            .map(|direction_index| self + HexVector::from_index(direction_index))
            .collect()
    }

    pub fn to_point(self) -> Point2 {
        Point2::new(
            (self.q as f32 + self.r as f32 / 2.0) * HEX_WIDTH,
            self.r as f32 * HEX_HEIGHT * 3.0 / 4.0,
        )
    }
}

// 0..6
pub type DirectionIndex = i32;

impl HexVector {
    pub fn new(q: i32, r: i32) -> HexVector {
        HexVector {q, r}
    }

    // right, then counter-clockwise
    pub fn from_index(direction_index: DirectionIndex) -> HexVector {
        match direction_index {
            0 => HexVector::new( 1,  0),
            1 => HexVector::new( 1, -1),
            2 => HexVector::new( 0, -1),
            3 => HexVector::new(-1,  0),
            4 => HexVector::new(-1,  1),
            5 => HexVector::new( 0,  1),
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub fn to_vector(self) -> Vector2 {
        Vector2::new(
            (self.q as f32 + self.r as f32 / 2.0) * HEX_WIDTH,
            self.r as f32 * HEX_HEIGHT * 3.0 / 4.0,
        )
    }

    #[allow(dead_code)]
    pub fn to_rotation(self) -> f32 {
        let v = self.to_vector();
        f32::atan2(v.y, v.x)
    }
}
