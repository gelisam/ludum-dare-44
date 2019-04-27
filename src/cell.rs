extern crate core;
extern crate ggez;
extern crate rand;

#[derive(Debug)]
enum IntOrInfinite {
    Infinite,
    Finite(u32)
}

#[derive(Debug)]
enum Gift {
    Leaves,
    Flowers,
    Berries,
	Nuts,
}

#[derive(Debug)]
struct Cell {
	branch_strain_current: usize,
    branch_upgrade: u8,
    gift_upgrade: u8,
	gift: Option<Gift>,
}

impl Cell {
    fn branch_strain_maximum(&self) -> IntOrInfinite {
        match self.branch_upgrade {
            0 => IntOrInfinite::Finite(5),
            1 => IntOrInfinite::Finite(25),
            2 => IntOrInfinite::Finite(125),
            4 => IntOrInfinite::Infinite,
            _ => unreachable!(),
        }
    }
}
