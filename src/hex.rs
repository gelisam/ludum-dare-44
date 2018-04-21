// using "pointy-topped axial coordinates":
// https://www.redblobgames.com/grids/hexagons/#coordinates-axial
#[derive(Debug, Clone, Copy)]
struct HexTile {
    q: i32,
    r: i32,
}

impl HexTile {
    fn new(q: i32, r: i32) -> HexTile {
        HexTile {q, r}
    }

    // right, then counter-clockwise
    fn neighbours(&self) -> [HexTile; 6] {
        [
            HexTile::new(self.q+1, self.r+0),
            HexTile::new(self.q+1, self.r-1),
            HexTile::new(self.q+0, self.r-1),
            HexTile::new(self.q-1, self.r+0),
            HexTile::new(self.q-1, self.r+1),
            HexTile::new(self.q+0, self.r+1),
        ]
    }
}
