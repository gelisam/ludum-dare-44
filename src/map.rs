use ggez::{GameResult, Context};
use ggez::graphics::Image;
use std::collections::HashMap;
use rand::{self,Rng};

use bomb::{self,Bomb,FuseLength};
use car::{self,Car};
use center::draw_centered;
use checkpoint::*;
use globals::*;
use hex::{HexPoint, HexVector};


#[derive(Debug)]
pub struct Assets {
    checkpoint_line: Image,
    finish_line:     Image,
    future_bomb:     Image,
    wall:            Image,
}

pub fn load_assets(ctx: &mut Context) -> GameResult<Assets> {
    Ok(
        Assets {
            checkpoint_line: Image::new(ctx, "/checkpoint-line.png")?,
            finish_line:     Image::new(ctx, "/finish-line.png")?,
            future_bomb:     Image::new(ctx, "/future-bomb.png")?,
            wall:            Image::new(ctx, "/wall.png")?,
        }
    )
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum FloorContents {
    CheckpointLine(f32),
    FinishLine(f32),
    FutureBomb(FuseLength),
}

impl FloorContents {
    pub fn draw(self, ctx: &mut Context, assets: &Assets, dest: HexPoint) -> GameResult<()> {
        match self {
            FloorContents::CheckpointLine(rotation) =>
                draw_centered(ctx, &assets.checkpoint_line, image_size(), dest.to_point(), rotation),
            FloorContents::FinishLine(rotation) =>
                draw_centered(ctx, &assets.finish_line, image_size(), dest.to_point(), rotation),
            FloorContents::FutureBomb(_) =>
                draw_centered(ctx, &assets.future_bomb, image_size(), dest.to_point(), 0.0),
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
    pub fn get(&self, hex_point: HexPoint) -> Option<CellContents> {
        let distance_from_center = hex_point.distance_from_center();
        if distance_from_center > CENTRAL_OBSTACLE_RADIUS
        && distance_from_center <= MAP_RADIUS
        {
            self.cells.get(&hex_point).map (|x| *x)
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
        let mut bomb_changes: Vec<(HexPoint, Option<Bomb>)> = Vec::with_capacity(100);
        let mut future_bomb_changes: Vec<(HexPoint, FuseLength)> = Vec::with_capacity(100);

        for (hex_point, cell_contents) in self.cells.iter() {
            match cell_contents {
                &CellContents::Bomb(bomb) => bomb_changes.push((*hex_point, bomb.decrement())),
                _ => (),
            }
        }
        for (hex_point, option_bomb) in bomb_changes {
            self.remove(hex_point);
            match option_bomb {
                Some(bomb) => {
                    self.insert(hex_point, CellContents::Bomb(bomb));
                },
                None => {
                    match self.random_available_spot() {
                        Some(new_spot) => {
                            future_bomb_changes.push((new_spot, MAX_FUSE_LENGTH));
                        },
                        None => (),
                    }
                },
            }
        }

        for (hex_point, floor_contents) in self.floor.iter() {
            match floor_contents {
                &FloorContents::FutureBomb(fuse_length) => {
                    future_bomb_changes.push((*hex_point, fuse_length - 1));
                },
                _ => (),
            }
        }
        for (hex_point, fuse_length) in future_bomb_changes {
            self.floor.remove(&hex_point);
            self.insert_bomb(hex_point, fuse_length);
        }
    }

    pub fn insert(&mut self, hex_point: HexPoint, cell_contents: CellContents) {
        self.cells.insert(hex_point, cell_contents);
    }

    pub fn insert_bomb(&mut self, hex_point: HexPoint, fuse_length: FuseLength) {
        if fuse_length <= 3 {
            match self.get(hex_point) {
                None => {
                    self.insert(hex_point, CellContents::Bomb(Bomb::new(fuse_length)));
                },
                Some(_) => {
                    // delay the appearance of the bomb until the obstacle moves away
                    self.floor.insert(hex_point, FloorContents::FutureBomb(fuse_length + 1));
                },
            }
        } else {
            self.floor.insert(hex_point, FloorContents::FutureBomb(fuse_length));
        }
    }

    pub fn remove(&mut self, hex_point: HexPoint) {
        self.cells.remove(&hex_point);
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
