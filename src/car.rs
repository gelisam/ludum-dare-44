use hex::HexVector;


// 0..3
pub type CarNumber = usize;

#[derive(Clone, Copy, Debug)]
pub struct Car {
    pub car_number: CarNumber,
    pub direction: HexVector,
}

impl Car {
    pub fn new(car_number: CarNumber, direction: HexVector) -> Car {
        Car {car_number, direction}
    }
}
