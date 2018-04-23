use ggez::{GameResult, Context};
use ggez::graphics::Image;
use std::collections::HashMap;
use rand::{self,Rng};

use bomb::{self,Bomb};
use car::{self,Car};
use center::draw_centered;
use checkpoint::*;
use globals::*;
use hex::{HexPoint, HexVector};


#[derive(Debug)]
pub struct Assets {
    bomb3:           Image,
    bomb2:           Image,
    bomb1:           Image,
    bomb0:           Image,
    checkpoint_line: Image,
    finish_line:     Image,
    wall:            Image,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(
        Assets {
            bomb3:           Image::new(ctx, "/bomb3.png")?,
            bomb2:           Image::new(ctx, "/bomb2.png")?,
            bomb1:           Image::new(ctx, "/bomb1.png")?,
            bomb0:           Image::new(ctx, "/bomb0.png")?,
            checkpoint_line: Image::new(ctx, "/checkpoint-line.png")?,
            finish_line:     Image::new(ctx, "/finish-line.png")?,
            wall:            Image::new(ctx, "/wall.png")?,
        }
    )
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
    Bomb(Bomb),
    Car(Car),
    Wall,
}

impl CellContents {
    pub fn draw(self, ctx: &mut Context, assets: &Assets, bomb_assets: &bomb::Assets, car_assets: &car::Assets, dest: HexPoint) -> GameResult<()> {
        match self {
            CellContents::Bomb(bomb) =>
                bomb.draw(ctx, bomb_assets, dest.to_point()),
            CellContents::Car(car) =>
                car.draw(ctx, car_assets, dest.to_point()),
            CellContents::Wall =>
                draw_centered(ctx, &assets.wall, image_size(), dest.to_point(), 0.0),
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

    // a location whose floor and cell are both empty.
    pub fn is_available(&self, hex_point: HexPoint) -> bool {
        self.floor.get(&hex_point).is_none() && self.get(hex_point).is_none()
    }

    pub fn random_available_spot(&self) -> Option<HexPoint> {
        let mut spots: Vec<HexPoint> = Vec::with_capacity(100);
        for r in -MAP_RADIUS..MAP_RADIUS+1 {
            for q in -MAP_RADIUS..MAP_RADIUS+1 {
                let hex_point = HexPoint::new(q, r);
                if self.is_available(hex_point) {
                     spots.push(hex_point);
                }
            }
        }

        if spots.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let i = rng.gen_range(0, spots.len());
            Some(spots[i])
        }
    }

    pub fn decrement_all_bombs(&mut self) {
        let mut changes: Vec<(HexPoint, Option<Bomb>)> = Vec::with_capacity(100);
        for (hex_point, cell_contents) in self.cells.iter() {
            match cell_contents {
                &CellContents::Bomb(bomb) => changes.push((*hex_point, bomb.decrement())),
                _ => (),
            }
        }
        for (hex_point, option_bomb) in changes {
            self.remove(hex_point);
            if let Some(bomb) = option_bomb {
                self.insert(hex_point, CellContents::Bomb(bomb));
            }
        }
    }

    pub fn insert(&mut self, index: HexPoint, cell_contents: CellContents) {
        self.cells.insert(index, cell_contents);
    }

    pub fn remove(&mut self, index: HexPoint) {
        self.cells.remove(&index);
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, bomb_assets: &bomb::Assets, car_assets: &car::Assets) -> GameResult<()> {
        for (dest, floor_contents) in &self.floor {
            floor_contents.draw(ctx, assets, *dest)?;
        }

        for (dest, cell_contents) in &self.cells {
            cell_contents.draw(ctx, assets, bomb_assets, car_assets, *dest)?;
        }

        for r in -CENTRAL_OBSTACLE_RADIUS..CENTRAL_OBSTACLE_RADIUS+1 {
            for q in -CENTRAL_OBSTACLE_RADIUS..CENTRAL_OBSTACLE_RADIUS+1 {
                let hex_point = HexPoint::new(q, r);
                let distance_from_center = hex_point.distance_from_center();
                if distance_from_center == CENTRAL_OBSTACLE_RADIUS
                {
                    CellContents::Wall.draw(ctx, assets, bomb_assets, car_assets, hex_point)?;
                }
            }
        }

        Ok(())
    }
}
