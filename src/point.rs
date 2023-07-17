use rand::Rng;
use std::ops::Range;

pub struct Point<T = usize> {
    pub x: T,
    pub y: T,
}

impl Point<usize> {
    pub fn random(x_range: Range<usize>, y_range: Range<usize>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(x_range),
            y: rng.gen_range(y_range),
        }
    }
}
