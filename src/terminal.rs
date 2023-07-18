use std::io;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::board::Board;
use crate::tile;

pub struct Terminal {
    board: Board,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            _stdout: io::stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn run(&self) {
        for row in &self.board.tiles {
            let mut result: Vec<String> = Vec::new();
            for tile in row {
                match tile.value() {
                    tile::Value::Number(n) => result.push(n.to_string()),
                    tile::Value::Mine => result.push("X".to_string()),
                }
            }
            println!("{}\r", result.join(" "));
        }
    }
}
