
use core::ops::{Add,AddAssign,Mul,MulAssign};
use ggez::{GameResult, Context};
use ggez::graphics::{Color, Drawable, DrawMode, DrawParam, Mesh};
use glam::f32::Vec2;

use globals::*;


pub const HEX_RADIUS:         f32 = 16.0;
pub const HEX_WIDTH:          f32 = HEX_RADIUS * 2.0;
pub const HEX_HEIGHT:         f32 = HEX_RADIUS * SQRT_3;

pub const VISIBLE_HEX_RADIUS: f32 = HEX_RADIUS * 2.0;
pub const VISIBLE_HEX_WIDTH:  f32 = HEX_WIDTH  * 2.0;
pub const VISIBLE_HEX_HEIGHT: f32 = HEX_HEIGHT * 2.0;

pub const ORIGIN_X:           f32 = WINDOW_WIDTH as f32 / 2.0;
pub const ORIGIN_Y:           f32 = WINDOW_HEIGHT as f32 - 70.0;


#[derive(Debug)]
pub struct Assets {
    hex: Mesh,
}

fn load_polygon_asset(ctx: &mut Context, mode: DrawMode) -> GameResult<Mesh> {
    Mesh::new_polygon(
        ctx,
        mode,
        &[
            Vec2::new(0.5 * VISIBLE_HEX_RADIUS, VISIBLE_HEX_HEIGHT / 2.0),
            Vec2::new(VISIBLE_HEX_WIDTH / 2.0, 0.0),
            Vec2::new(0.5 * VISIBLE_HEX_RADIUS, -VISIBLE_HEX_HEIGHT / 2.0),
            Vec2::new(-0.5 * VISIBLE_HEX_RADIUS, -VISIBLE_HEX_HEIGHT / 2.0),
            Vec2::new(-VISIBLE_HEX_WIDTH / 2.0, 0.0),
            Vec2::new(-0.5 * VISIBLE_HEX_RADIUS, VISIBLE_HEX_HEIGHT / 2.0),
        ],
        Color::from_rgb(163, 186, 188)
    )
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(Assets {
        hex: load_polygon_asset(ctx, DrawMode::stroke(1.0))?,
    })
}

pub fn draw_hex_grid(ctx: &mut Context, assets: &Assets) -> GameResult<()> {
    for q in -10..=10 {
        for r in -20..=0 {
            if let Some(InBoundsPoint::GiftPoint(gift_point)) = HexPoint::new(q, r).is_in_bounds() {
                assets.hex.draw(
                    ctx,
                    DrawParam::default()
                        .dest(gift_point.to_point())
                )?;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BranchPoint {
    pub hex_point: HexPoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GiftPoint {
    pub hex_point: HexPoint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InBoundsPoint {
    BranchPoint(BranchPoint),
    GiftPoint(GiftPoint),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Orientation {
    Vert,     // |
    Diag,     // /
    AntiDiag, // \
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

    fn would_be_cell_center(self) -> bool {
        self.q % 2 == 0 && self.r % 2 == 0
    }

    pub fn is_in_bounds(self) -> Option<InBoundsPoint> {
        if self.r <= 0 && self.s() >= 0 && self.q >= -10 && self.q <= 10 && self.y() + (if (self.q + 100) % 4 == 2 {1} else {0}) >= -33 && self.s() < 21 {
            if self.would_be_cell_center() {
                Some(InBoundsPoint::GiftPoint(GiftPoint::new(self)))
            } else {
                Some(InBoundsPoint::BranchPoint(BranchPoint::new(self)))
            }
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn neighbours(self) -> Vec<HexPoint> {
        (0..6)
            .map(|direction_index| self + HexVector::from_index(direction_index))
            .collect()
    }

    #[allow(dead_code)]
    pub fn neighbours2(self) -> Vec<HexPoint> {
        (0..6)
            .map(|direction_index| self + HexVector::from_index(direction_index) * 2)
            .collect()
    }

    pub fn from_point(point: Vec2) -> HexPoint {
        let q = (point.x - ORIGIN_X) * 4.0 / 3.0 / HEX_WIDTH;
        let r = (point.y - ORIGIN_Y) / HEX_HEIGHT - q / 2.0;
        HexPoint {
            q: q.round() as i32,
            r: r.round() as i32,
        }
    }

    pub fn to_point(self) -> Vec2 {
        Vec2::new(
            ORIGIN_X + self.q as f32 * HEX_WIDTH * 3.0 / 4.0,
            ORIGIN_Y + (self.r as f32 + self.q as f32 / 2.0) * HEX_HEIGHT,
        )
    }
}

// 0..6
pub type DirectionIndex = i32;

impl HexVector {
    pub fn new(q: i32, r: i32) -> HexVector {
        HexVector {q, r}
    }

    // up, then counter-clockwise
    pub fn from_index(direction_index: DirectionIndex) -> HexVector {
        match direction_index {
            0 => HexVector::new( 0, -1),
            1 => HexVector::new(-1,  0),
            2 => HexVector::new(-1,  1),
            3 => HexVector::new( 0,  1),
            4 => HexVector::new( 1,  0),
            5 => HexVector::new( 1, -1),
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub fn to_vector(self) -> Vec2 {
        Vec2::new(
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

impl BranchPoint {
    pub fn new(hex_point: HexPoint) -> BranchPoint {
        BranchPoint {hex_point}
    }

    pub fn to_point(self) -> Vec2 {
        self.hex_point.to_point()
    }

    pub fn orientation(self) -> Orientation {
        if (self.hex_point + HexVector::from_index(0)).would_be_cell_center() {
            Orientation::Vert
        } else if (self.hex_point + HexVector::from_index(5)).would_be_cell_center() {
            Orientation::Diag
        } else {
            Orientation::AntiDiag
        }
    }

    #[allow(dead_code)]
    pub fn gift_neighbours(self) -> Vec<GiftPoint> {
        self.hex_point.neighbours()
            .iter()
            .filter_map(|p| p.is_in_bounds())
            .filter_map(|p| match p {
                InBoundsPoint::GiftPoint(gift_point) => Some(gift_point),
                _ => None
            })
            .collect()
    }
}

impl GiftPoint {
    pub fn new(hex_point: HexPoint) -> GiftPoint {
        GiftPoint {hex_point}
    }

    pub fn to_point(self) -> Vec2 {
        self.hex_point.to_point()
    }

    #[allow(dead_code)]
    pub fn branch_neighbours(self) -> Vec<BranchPoint> {
        self.hex_point.neighbours()
            .iter()
            .filter_map(|p| p.is_in_bounds())
            .filter_map(|p| match p {
                InBoundsPoint::BranchPoint(branch_point) => Some(branch_point),
                _ => None
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn gift_neighbours(self) -> Vec<GiftPoint> {
        self.hex_point.neighbours2()
            .iter()
            .filter_map(|p| p.is_in_bounds())
            .filter_map(|p| match p {
                InBoundsPoint::GiftPoint(gift_point) => Some(gift_point),
                _ => None
            })
            .collect()
    }
}

impl InBoundsPoint {
    pub fn to_point(self) -> Vec2 {
        match self {
            InBoundsPoint::BranchPoint(branch_point) => branch_point.to_point(),
            InBoundsPoint::GiftPoint(gift_point)     => gift_point.to_point(),
        }
    }
}
