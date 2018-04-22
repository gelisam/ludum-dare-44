use car::Car;
use checkpoint::*;
use hex::{DirectionIndex, HexPoint, HexVector};
use map::{CellContents,Map};

#[derive(Clone, Copy, Debug)]
pub struct Racer {
    pub checkpoint: Checkpoint,
    pub direction_index: DirectionIndex,
    pub number: usize,
    pub position: HexPoint,
}

impl Racer {
    pub fn new(number: usize, position: HexPoint) -> Racer {
        let checkpoint = point_to_section(position);
        let direction_index = forward(position);
        Racer {checkpoint, direction_index, number, position}
    }

    pub fn turn_left(self) -> Racer {
        Racer {
            direction_index: (self.direction_index + 1) % 6,
            ..self
        }
    }

    pub fn turn_right(self) -> Racer {
        Racer {
            direction_index: (self.direction_index + 5) % 6,
            ..self
        }
    }

    fn move_to(self, position: HexPoint) -> Racer {
        let checkpoint = update_checkpoint(self.checkpoint, position);
        Racer {
            position,
            checkpoint,
            ..self
        }
    }

    pub fn go_forward(self) -> Racer {
        self.move_to(self.position + HexVector::from_index(self.direction_index))
    }

    pub fn go_backwards(self) -> Racer {
        self.move_to(self.position + HexVector::from_index(self.direction_index) * -1)
    }

    pub fn go_back_to_checkpoint(self, map: &Map) -> Racer {
        self.move_to(map.find_spot_at_checkpoint(self.checkpoint))
    }

    fn car(self) -> Car {
        Car::new(HexVector::from_index(self.direction_index))
    }

    pub fn remove(self, map: &mut Map) {
        map.remove(self.position);
    }

    pub fn insert(self, map: &mut Map) {
        map.insert(self.position, CellContents::Car(self.car()));
    }
}
