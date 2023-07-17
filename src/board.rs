use crate::tile;

pub struct Board {
    tiles: Vec<Vec<tile::Tile>>,
    height: usize,
    width: usize,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let tiles = vec![vec![tile::Tile::default(); height]; width];

        Self {
            tiles,
            height,
            width,
        }
    }
}
