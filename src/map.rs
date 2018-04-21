use ggez::{GameResult, Context};
use ggez::graphics::{Drawable, Font, Image, Text, Vector2};
use std::collections::HashMap;

use globals::*;
use hex::{HEX_WIDTH, HEX_HEIGHT, HexPoint, HexVector};
use text;


#[derive(Debug)]
pub struct Assets {
    bonus_box:       Text,
    car:             Image,
    checkpoint_line: Text,
    finish_line:     Text,
    obstacle:        Text,
    wall:            Text,
}

pub fn load_assets(ctx: &mut Context, font: &Font) -> GameResult<Assets> {
    Ok(
        Assets {
            bonus_box:       Text::new(ctx, "?", &font)?,
            car:             Image::new(ctx, "/car.png")?,
            checkpoint_line: Text::new(ctx, ".", &font)?,
            finish_line:     Text::new(ctx, ":", &font)?,
            obstacle:        Text::new(ctx, "@", &font)?,
            wall:            Text::new(ctx, "#", &font)?,
        }
    )
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum FloorContents {
    CheckpointLine,
    FinishLine,
}

impl FloorContents {
    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: HexPoint) -> GameResult<()> {
        let text: &Text = match self {
            FloorContents::CheckpointLine => &assets.checkpoint_line,
            FloorContents::FinishLine     => &assets.finish_line,
        };
        text::draw_centered_text(ctx, text, dest.to_point(), 0.0)
    }
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum CellContents {
    BonusBox,
    Car,
    Obstacle,
    Wall,
}

impl CellContents {
    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: HexPoint) -> GameResult<()> {
        match self {
            CellContents::Car =>
                assets.car.draw(ctx, dest.to_point() - Vector2::new(HEX_WIDTH / 2.0, HEX_HEIGHT / 2.0), 0.0),
            _ => {
                let text: &Text = match self {
                    CellContents::BonusBox       => &assets.bonus_box,
                    CellContents::Car            => unreachable!(),
                    CellContents::Obstacle       => &assets.obstacle,
                    CellContents::Wall           => &assets.wall,
                };
                let rotation: f32 = match self {
                    CellContents::Car => PI, // I want the pointy bit of the "V" to point upwards
                    _   => 0.0,
                };
                text::draw_centered_text(ctx, text, dest.to_point(), rotation)
            }
        }
    }
}


#[derive(Clone, Debug)]
pub struct Map {
    cells: HashMap<HexPoint, CellContents>,
    floor: HashMap<HexPoint, FloorContents>,
    car: HexPoint,
}

impl Map {
    pub fn load() -> Map {
        const CENTRAL_OBSTACLE_RADIUS: i32 = 2;

        let mut cells: HashMap<HexPoint, CellContents> = HashMap::with_capacity(100);
        for q in -CENTRAL_OBSTACLE_RADIUS..CENTRAL_OBSTACLE_RADIUS+1 {
            for r in -CENTRAL_OBSTACLE_RADIUS..CENTRAL_OBSTACLE_RADIUS+1 {
                if i32::abs(q + r) <= CENTRAL_OBSTACLE_RADIUS {
                    cells.insert(HexPoint::new(q, r), CellContents::Wall);
                }
            }
        }

        let mut floor: HashMap<HexPoint, FloorContents> = HashMap::with_capacity(100);
        let directions = HexVector::all_directions();
        for q in CENTRAL_OBSTACLE_RADIUS+1..MAP_RADIUS+1 {
            for i in 0..6 {
                floor.insert(
                    HexPoint::new(0, 0) + directions[i] * q,
                    if i == 0 {
                        FloorContents::FinishLine
                    } else {
                        FloorContents::CheckpointLine
                    },
                );
            }
        }

        let car = HexPoint::new(CENTRAL_OBSTACLE_RADIUS+2, 0);
        cells.insert(car, CellContents::Car);

        Map {cells, floor, car}
    }

    pub fn go_forward(&mut self) {
        self.cells.remove(&self.car);
        self.car += self.car.forward();
        self.cells.insert(self.car, CellContents::Car);
    }

    #[allow(dead_code)]
    pub fn get(&self, index: HexPoint) -> Option<CellContents> {
        if index.q.abs() <= MAP_RADIUS
        && index.r.abs() <= MAP_RADIUS
        && (index.q + index.r).abs() <= MAP_RADIUS
        {
            self.cells.get(&index).map (|x| *x)
        } else {
            Some(CellContents::Wall)
        }
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        for (dest, floor_contents) in &self.floor {
            floor_contents.draw(ctx, assets, *dest)?;
        }

        for (dest, cell_contents) in &self.cells {
            cell_contents.draw(ctx, assets, *dest)?;
        }

        Ok(())
    }
}
