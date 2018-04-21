use hex::HexVector;

#[derive(Clone, Copy, Debug)]
pub struct Car {
    pub direction: HexVector,
}

impl Car {
    pub fn new(direction: HexVector) -> Car {
        Car {direction}
    }

    pub fn rotation(self) -> f32 {
        let v = self.direction.to_vector();
        f32::atan2(v.y, v.x)
    }
}
