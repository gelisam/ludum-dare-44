use hex::HexVector;

#[derive(Clone, Copy, Debug)]
pub struct Car {
    pub direction: HexVector,
}

impl Car {
    pub fn new(direction: HexVector) -> Car {
        Car {direction}
    }
}
