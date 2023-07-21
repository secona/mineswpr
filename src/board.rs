use crate::{point::Point, tile};

#[derive(Debug)]
pub struct Board {
    pub tiles: Vec<Vec<tile::Tile>>,
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
}

impl Board {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        let tiles = vec![vec![tile::Tile::default(); width]; height];
        let mut mine_points: Vec<Point> = Vec::new();

        let mut result = Self {
            tiles,
            height,
            width,
            mine_count,
        };

        while mine_points.len() < mine_count {
            let random = Point::random(0..width, 0..height);

            if !mine_points.contains(&random) {
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

                mine_points.push(random);
            }
        }

        result
    }

    pub fn restart(&mut self) {
        *self = Board::new(self.width, self.height, self.mine_count);
    }

    pub fn has_won(&self) -> bool {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                if let tile::Value::Number(_) = tile.value() {
                    if let tile::State::Opened = tile.state() {
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn tile_at(&mut self, point: &Point) -> &mut tile::Tile {
        &mut self.tiles[point.y][point.x]
    }

    pub fn open_tile(&mut self, point: &Point) -> Result<(), ()> {
        let tile = self.tile_at(point);
        match tile.open() {
            Ok(_) => {
                if let tile::Value::Number(0) = tile.value() {
                    for neighbor in point.neighbors() {
                        if neighbor.x >= self.width || neighbor.y >= self.height {
                            continue;
                        }

                        if let tile::State::Closed = self.tile_at(&neighbor).state() {
                            self.open_tile(&neighbor)?;
                        }
                    }
                }
            }
            Err(kind) => match kind {
                tile::OpenError::Mine => return Err(()),
                _ => {}
            },
        };

        Ok(())
    }
}
