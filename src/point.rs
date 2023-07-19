use rand::Rng;
use std::ops::{Range, RangeInclusive};

#[derive(Default, PartialEq)]
pub struct Point<T = usize> {
    pub x: T,
    pub y: T,
}

impl Point<usize> {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn random(x_range: Range<usize>, y_range: Range<usize>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(x_range),
            y: rng.gen_range(y_range),
        }
    }

    pub fn neighbors(&self) -> Vec<Point> {
        let mut result = Vec::new();
        for i in (-1..=1) as RangeInclusive<i32> {
            for j in (-1..=1) as RangeInclusive<i32> {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = match self.x.checked_add_signed(i as isize) {
                    Some(value) => value,
                    None => continue,
                };

                let y = match self.y.checked_add_signed(j as isize) {
                    Some(value) => value,
                    None => continue,
                };

                result.push(Point { x, y })
            }
        }
        result
    }
}
