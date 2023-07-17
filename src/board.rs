use crate::{point::Point, tile};

#[derive(Debug)]
pub struct Board {
    tiles: Vec<Vec<tile::Tile>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let mut tiles = vec![vec![tile::Tile::default(); width]; height];

        for _ in 0..mine_count {
            let random = Point::random(0..width, 0..height);
            tiles[random.x][random.y].replace_value(tile::Value::Mine);

            for neighbor in random.neighbors() {
                if neighbor.x >= width || neighbor.y >= height {
                    continue;
                }

                let tile = &mut tiles[neighbor.x][neighbor.y];
                if let tile::Value::Number(num) = tile.value() {
                    tile.replace_value(tile::Value::Number(num + 1))
                }
            }
        }

        Self {
            tiles,
            height,
            width,
        }
    }
}
