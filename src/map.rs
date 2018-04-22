use ggez::{GameResult, Context};
use ggez::graphics::{Font, Image, Text, Vector2};
use std::collections::HashMap;

use car::Car;
use center::draw_centered;
use checkpoint::*;
use globals::*;
use hex::{HexPoint, HexVector};
use text;


#[derive(Debug)]
pub struct Assets {
    bonus_box:       Text,
    car:             Image,
    checkpoint_line: Image,
    finish_line:     Image,
    obstacle:        Text,
    wall:            Image,
}

pub fn load_assets(ctx: &mut Context, font: &Font) -> GameResult<Assets> {
    Ok(
        Assets {
            bonus_box:       Text::new(ctx, "?", &font)?,
            car:             Image::new(ctx, "/car1.png")?,
            checkpoint_line: Image::new(ctx, "/checkpoint-line.png")?,
            finish_line:     Image::new(ctx, "/finish-line.png")?,
            obstacle:        Text::new(ctx, "@", &font)?,
            wall:            Image::new(ctx, "/wall.png")?,
        }
    )
}

fn image_size() -> Vector2 {
    Vector2::new(56.0, 102.0)
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum FloorContents {
    CheckpointLine(f32),
    FinishLine(f32),
}

impl FloorContents {
    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: HexPoint) -> GameResult<()> {
        match self {
            FloorContents::CheckpointLine(rotation) =>
                draw_centered(ctx, &assets.checkpoint_line, image_size(), dest.to_point(), rotation),
            FloorContents::FinishLine(rotation) =>
                draw_centered(ctx, &assets.finish_line, image_size(), dest.to_point(), rotation),
        }
    }
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum CellContents {
    BonusBox,
    Car(Car),
    Obstacle,
    Wall,
}

impl CellContents {
    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: HexPoint) -> GameResult<()> {
        match self {
            CellContents::Car(car) =>
                draw_centered(ctx, &assets.car, image_size(), dest.to_point(), car.direction.to_rotation()),
            CellContents::Wall =>
                draw_centered(ctx, &assets.wall, image_size(), dest.to_point(), 0.0),
            _ => {
                let text: &Text = match self {
                    CellContents::BonusBox       => &assets.bonus_box,
                    CellContents::Car(_)         => unreachable!(),
                    CellContents::Obstacle       => &assets.obstacle,
                    CellContents::Wall           => unreachable!(),
                };
                text::draw_centered_text(ctx, text, dest.to_point(), 0.0)
            }
        }
    }
}


#[derive(Clone, Debug)]
pub struct Map {
    cells: HashMap<HexPoint, CellContents>,
    floor: HashMap<HexPoint, FloorContents>,
}

impl Map {
    pub fn load() -> Map {
        let cells: HashMap<HexPoint, CellContents> = HashMap::with_capacity(100);

        let mut floor: HashMap<HexPoint, FloorContents> = HashMap::with_capacity(100);
        for distance in CENTRAL_OBSTACLE_RADIUS+1..MAP_RADIUS+1 {
            for direction_index in 0..6 {
                floor.insert(
                    HexPoint::new(0, 0) + HexVector::from_index(direction_index) * distance,
                    if direction_index == 0 {
                        FloorContents::FinishLine(HexVector::from_index(direction_index).to_rotation())
                    } else {
                        FloorContents::CheckpointLine(HexVector::from_index(direction_index).to_rotation())
                    },
                );
            }
        }

        Map {cells, floor}
    }

    #[allow(dead_code)]
    pub fn get(&self, index: HexPoint) -> Option<CellContents> {
        let distance_from_center = index.distance_from_center();
        if distance_from_center > CENTRAL_OBSTACLE_RADIUS
        && distance_from_center <= MAP_RADIUS
        {
            self.cells.get(&index).map (|x| *x)
        } else {
            Some(CellContents::Wall)
        }
    }

    // prefer an empty spot, or a non-car spot if neccessary.
    pub fn find_spot_at_checkpoint(&self, checkpoint: Checkpoint) -> HexPoint {
        let section = checkpoint_to_section(checkpoint);
        let direction = HexVector::from_index(section);

        for distance in (CENTRAL_OBSTACLE_RADIUS+1..MAP_RADIUS+1).rev() {
            let hex_point = HexPoint::new(0, 0) + direction * distance;
            match self.get(hex_point) {
                None => return hex_point,
                Some(_) => (),
            }
        }

        for distance in (CENTRAL_OBSTACLE_RADIUS+1..MAP_RADIUS+1).rev() {
            let hex_point = HexPoint::new(0, 0) + direction * distance;
            match self.get(hex_point) {
                Some(CellContents::Car(_)) => (),
                Some(_) => return hex_point,
                None => unreachable!(),
            }
        }

        panic!("no spots left!"); // should not happen unless there are more than 3 cars in the game
    }

    pub fn insert(&mut self, index: HexPoint, cell_contents: CellContents) {
        self.cells.insert(index, cell_contents);
    }

    pub fn remove(&mut self, index: HexPoint) {
        self.cells.remove(&index);
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets) -> GameResult<()> {
        for (dest, floor_contents) in &self.floor {
            floor_contents.draw(ctx, assets, *dest)?;
        }

        for (dest, cell_contents) in &self.cells {
            cell_contents.draw(ctx, assets, *dest)?;
        }

        for r in -CENTRAL_OBSTACLE_RADIUS..CENTRAL_OBSTACLE_RADIUS+1 {
            for q in -CENTRAL_OBSTACLE_RADIUS..CENTRAL_OBSTACLE_RADIUS+1 {
                let hex_point = HexPoint::new(q, r);
                let distance_from_center = hex_point.distance_from_center();
                if distance_from_center == CENTRAL_OBSTACLE_RADIUS
                {
                    CellContents::Wall.draw(ctx, assets, hex_point)?;
                }
            }
        }

        Ok(())
    }
}
