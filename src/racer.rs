use car::{Car,CarNumber};
use checkpoint::*;
use hex::{DirectionIndex, HexPoint, HexVector};
use map::{CellContents,Map};

#[derive(Clone, Copy, Debug)]
pub struct Racer {
    pub car_number: CarNumber,
    pub checkpoint: Checkpoint,
    pub direction_index: DirectionIndex,
    pub position: HexPoint,
}

impl Racer {
    pub fn new(car_number: CarNumber, position: HexPoint) -> Racer {
        let checkpoint = point_to_section(position);
        let direction_index = forward(position);
        Racer {car_number, checkpoint, direction_index, position}
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

    fn move_to(self, position: HexPoint, map: &Map) -> Option<Racer> {
        match map.get(position) {
            None => {
                let checkpoint = update_checkpoint(self.checkpoint, position);
                Some(Racer {
                    position,
                    checkpoint,
                    ..self
                })
            },
            Some(_) => None,
        }
    }

    pub fn go_forward(self, map: &Map) -> Option<Racer> {
        self.move_to(self.position + HexVector::from_index(self.direction_index), map)
    }

    pub fn go_backwards(self, map: &Map) -> Option<Racer> {
        self.move_to(self.position + HexVector::from_index(self.direction_index) * -1, map)
    }

    pub fn go_back_to_checkpoint(self, map: &Map) -> Racer {
        self.move_to(map.find_spot_at_checkpoint(self.checkpoint), map).unwrap()
    }

    pub fn to_car(self) -> Car {
        Car::new(
            self.car_number,
            HexVector::from_index(self.direction_index),
        )
    }

    pub fn remove(self, map: &mut Map) {
        map.remove(self.position);
    }

    pub fn insert(self, map: &mut Map) {
        map.insert(self.position, CellContents::Car(self.to_car()));
    }
}
