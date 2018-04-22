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
    car_position: HexPoint,
    car_checkpoint: Checkpoint,
}

impl Map {
    pub fn load() -> Map {
        let mut cells: HashMap<HexPoint, CellContents> = HashMap::with_capacity(100);

        let mut floor: HashMap<HexPoint, FloorContents> = HashMap::with_capacity(100);
        let directions = HexVector::all_directions();
        for q in CENTRAL_OBSTACLE_RADIUS+1..MAP_RADIUS+1 {
            for i in 0..6 {
                floor.insert(
                    HexPoint::new(0, 0) + directions[i] * q,
                    if i == 0 {
                        FloorContents::FinishLine(directions[i].to_rotation())
                    } else {
                        FloorContents::CheckpointLine(directions[i].to_rotation())
                    },
                );
            }
        }

        let car_position = HexPoint::new(CENTRAL_OBSTACLE_RADIUS+2, 0);
        let car_checkpoint = 0;
        cells.insert(car_position, CellContents::Car(Car::new(forward(car_position))));

        Map {cells, floor, car_position, car_checkpoint}
    }

    pub fn go_forward(&mut self) {
        self.cells.remove(&self.car_position);
        self.car_position += forward(self.car_position);
        self.cells.insert(self.car_position, CellContents::Car(Car::new(forward(self.car_position))));

        self.car_checkpoint = update_checkpoint(self.car_checkpoint, self.car_position);
        println!(
            "section {:?}, checkpoint {:?}, lap {:?}",
            point_to_section(self.car_position),
            self.car_checkpoint,
            lap(self.car_checkpoint),
        );
    }

    pub fn go_backwards(&mut self) {
        self.cells.remove(&self.car_position);
        self.car_position += backward(self.car_position);
        self.cells.insert(self.car_position, CellContents::Car(Car::new(forward(self.car_position))));

        self.car_checkpoint = update_checkpoint(self.car_checkpoint, self.car_position);
        println!(
            "section {:?}, checkpoint {:?}, lap {:?}",
            point_to_section(self.car_position),
            self.car_checkpoint,
            lap(self.car_checkpoint),
        );
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
