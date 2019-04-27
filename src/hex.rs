extern crate nalgebra;

use core::ops::{Add,AddAssign,Mul,MulAssign};
use ggez::{GameResult, Context};
use ggez::graphics::{self, Color, Drawable, DrawMode, Point2, Mesh, Vector2};

use globals::*;


pub const HEX_RADIUS:         f32 = 16.0;
pub const HEX_WIDTH:          f32 = HEX_RADIUS * 2.0;
pub const HEX_HEIGHT:         f32 = HEX_RADIUS * SQRT_3;
pub const VISIBLE_HEX_RADIUS: f32 = HEX_RADIUS * 2.0;
pub const VISIBLE_HEX_WIDTH:  f32 = HEX_WIDTH  * 2.0;
pub const VISIBLE_HEX_HEIGHT: f32 = HEX_HEIGHT * 2.0;


#[derive(Debug)]
pub struct Assets {
    hex: Mesh,
    dot: Mesh,
}

fn load_polygon_asset(ctx: &mut Context, mode: DrawMode) -> GameResult<Mesh> {
    Mesh::new_polygon(
        ctx,
        mode,
        &[
            Point2::new(0.5 * VISIBLE_HEX_RADIUS, VISIBLE_HEX_HEIGHT / 2.0),
            Point2::new(VISIBLE_HEX_WIDTH / 2.0, 0.0),
            Point2::new(0.5 * VISIBLE_HEX_RADIUS, -VISIBLE_HEX_HEIGHT / 2.0),
            Point2::new(-0.5 * VISIBLE_HEX_RADIUS, -VISIBLE_HEX_HEIGHT / 2.0),
            Point2::new(-VISIBLE_HEX_WIDTH / 2.0, 0.0),
            Point2::new(-0.5 * VISIBLE_HEX_RADIUS, VISIBLE_HEX_HEIGHT / 2.0),
        ],
    )
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(Assets {
        hex: load_polygon_asset(ctx, DrawMode::Line(1.0))?,
        dot: Mesh::new_circle(ctx, DrawMode::Fill, Point2::new(0.0, 0.0), 3.0, 3.0)?,
    })
}

pub fn draw_hex_grid(ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    graphics::set_color(ctx, Color::from_rgb(163, 186, 188))?;
    for q in -8..=8 {
        for r in -20..=0 {
            let hex_point = HexPoint::new(q, r);
            if hex_point.is_cell_center() {
                assets.hex.draw(ctx, hex_point.to_point(), 0.0)?;
            }
        }
    }

    graphics::set_color(ctx, Color::from_rgb(163, 186, 188))?;
    for q in -8..=8 {
        for r in -20..=0 {
            let hex_point = HexPoint::new(q, r);
            if hex_point.is_in_bounds() {
                assets.dot.draw(ctx, hex_point.to_point(), 0.0)?;
            }
        }
    }

    Ok(())
}


// using "flat-topped axial coordinates":
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

    pub fn s(self) -> i32 {
        -self.q - self.r
    }

    pub fn y(self) -> i32 {
        self.r * 2 + self.q
    }

    pub fn is_in_bounds(self) -> bool {
        self.r <= 0 && self.s() >= 0 && self.q >= -8 && self.q <= 8 && self.y() >= -34 && self.s() < 21
    }

    pub fn is_cell_center(self) -> bool {
        self.is_in_bounds() && self.q % 2 == 0 && self.r % 2 == 0
    }

    pub fn is_cell_border(self) -> bool {
        self.is_in_bounds() && !self.is_cell_center()
    }


    #[allow(dead_code)]
    pub fn neighbours(self) -> Vec<HexPoint> {
        (0..6)
            .map(|direction_index| self + HexVector::from_index(direction_index))
            .collect()
    }

    pub fn to_point(self) -> Point2 {
        Point2::new(
            WINDOW_WIDTH as f32 / 2.0 + self.q as f32 * HEX_WIDTH * 3.0 / 4.0,
            WINDOW_HEIGHT as f32 - 70.0 + (self.r as f32 + self.q as f32 / 2.0) * HEX_HEIGHT,
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
