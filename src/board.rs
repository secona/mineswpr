use crate::{point::Point, tile};

#[derive(Debug)]
pub struct Board {
    tiles: Vec<Vec<tile::Tile>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let tiles = vec![vec![tile::Tile::default(); width]; height];

        let mut result = Self {
            tiles,
            height,
            width,
        };

        for _ in 0..mine_count {
            let random = Point::random(0..width, 0..height);
            result.tile_at(&random).replace_value(tile::Value::Mine);

            for neighbor in random.neighbors() {
                if neighbor.x >= width || neighbor.y >= height {
                    continue;
                }

                let tile = &mut result.tile_at(&neighbor);
                if let tile::Value::Number(num) = tile.value() {
                    tile.replace_value(tile::Value::Number(num + 1))
                }
            }
        }

        result
    }

    fn tile_at(&mut self, point: &Point) -> &mut tile::Tile {
        &mut self.tiles[point.x][point.y]
    }
}
