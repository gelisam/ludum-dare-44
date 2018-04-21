extern crate nalgebra;

use core::ops::Add;
use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, DrawMode, Point2, Mesh, Vector2};

use globals::*;


const SQRT_3: f32 = 1.7320508075688772;

const HEX_RADIUS: f32 = 32.0;
const HEX_WIDTH:  f32 = HEX_RADIUS * SQRT_3;
const HEX_HEIGHT: f32 = HEX_RADIUS * 2.0;

const MAP_RADIUS: i32 = 5;


pub type PolygonAsset = Mesh;

pub fn load_polygon_asset(ctx: &mut Context, mode: DrawMode) -> GameResult<PolygonAsset> {
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

pub fn draw_hex_grid(ctx: &mut Context, polygon_asset: &PolygonAsset) -> GameResult<()> {
    for q in -MAP_RADIUS..MAP_RADIUS+1 {
        for r in -MAP_RADIUS..MAP_RADIUS+1 {
            if (q + r).abs() <= MAP_RADIUS {
                polygon_asset.draw(
                    ctx,
                    HexPoint::new(q, r).to_point(),
                    0.0,
                )?;
            }
        }
    }

    Ok(())
}


// using "pointy-topped axial coordinates":
// https://www.redblobgames.com/grids/hexagons/#coordinates-axial
#[derive(Clone, Copy, Debug)]
pub struct HexPoint {
    q: i32,
    r: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct HexVector {
    q: i32,
    r: i32,
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

impl Add for HexVector {
    type Output = HexVector;

    fn add(self, other: HexVector) -> HexVector {
        HexVector::new(
            self.q + other.q,
            self.r + other.r,
        )
    }
}


impl HexPoint {
    fn new(q: i32, r: i32) -> HexPoint {
        HexPoint {q, r}
    }

    fn neighbours(self) -> [HexPoint; 6] {
        let dirs = HexVector::all_directions();
        [
            self + dirs[0],
            self + dirs[1],
            self + dirs[2],
            self + dirs[3],
            self + dirs[4],
            self + dirs[5],
        ]
    }

    fn to_point(self) -> Point2 {
        Point2::new(
            WINDOW_WIDTH  as f32 / 2.0 + (self.q as f32 + self.r as f32 / 2.0) * HEX_WIDTH,
            WINDOW_HEIGHT as f32 / 2.0 + self.r as f32 * HEX_HEIGHT * 3.0 / 4.0,
        )
    }
}

impl HexVector {
    fn new(q: i32, r: i32) -> HexVector {
        HexVector {q, r}
    }

    // right, then counter-clockwise
    fn all_directions() -> [HexVector; 6] {
        [
            HexVector::new( 1,  0),
            HexVector::new( 1, -1),
            HexVector::new( 0, -1),
            HexVector::new(-1,  0),
            HexVector::new(-1,  1),
            HexVector::new( 0,  1),
        ]
    }

    fn to_vector(self) -> Vector2 {
        Vector2::new(
            (self.q as f32 + self.r as f32 / 2.0) * HEX_WIDTH,
            self.r as f32 * HEX_HEIGHT * 3.0 / 4.0,
        )
    }
}
