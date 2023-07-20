use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::board::Board;
use crate::point::Point;
use crate::tile;

#[derive(PartialEq)]
struct Cursor {
    position: Point,
    max: Point,
}

impl From<&Board> for Cursor {
    fn from(value: &Board) -> Self {
        Self {
            position: Point::default(),
            max: Point::new(value.width, value.height),
        }
    }
}

impl Cursor {
    pub fn mut_move(&mut self, x: i32, y: i32) {
        self.position.x = self
            .position
            .x
            .saturating_add_signed(x as isize)
            .clamp(0, self.max.x - 1);

        self.position.y = self
            .position
            .y
            .saturating_add_signed(y as isize)
            .clamp(0, self.max.y - 1);
    }
}

pub struct Terminal {
    board: Board,
    should_quit: bool,
    width: usize,
    height: usize,
    cursor: Cursor,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn new(board: Board) -> Self {
        let term_size = termion::terminal_size().unwrap();

        Self {
            width: term_size.0 as usize,
            height: term_size.1 as usize,
            should_quit: false,
            cursor: Cursor::from(&board),
            board,
            _stdout: io::stdout().into_raw_mode().unwrap(),
        }
    }

    pub fn run(&mut self) {
        Terminal::clear_screen();
        Terminal::cursor_hide();

        loop {
            self.refresh_screen();

            if self.should_quit == true {
                break;
            }

            self.handle_keypress();
        }

        Terminal::cursor_show();
    }

    fn refresh_screen(&self) {
        Terminal::cursor_goto(1, 1);

        if self.should_quit {
            Terminal::cursor_goto(1, 1);
            Terminal::clear_after_cursor();
            println!("Goodbye.");
            return;
        }

        for (y, row) in self.board.tiles.iter().enumerate() {
            let mut result: Vec<String> = Vec::new();
            for (x, tile) in row.iter().enumerate() {
                let mut tile = match tile.state() {
                    tile::State::Opened => match tile.value() {
                        tile::Value::Number(n) => n.to_string(),
                        tile::Value::Mine => "X".to_string(),
                    },
                    tile::State::Closed => "?".to_string(),
                    tile::State::Flagged => "F".to_string(),
                };

                if self.cursor.position == Point::new(x, y) {
                    tile = Terminal::bg_color_str(&tile, termion::color::Yellow);
                }

                result.push(tile);
            }
            let result = result.join(" ");
            println!("{}{}\r", result, " ".repeat(self.width - result.len()));
        }

        let full_width_spaces = " ".repeat(self.width);
        for _ in 0..(self.height - self.board.height - 1) {
            println!("{}\r", full_width_spaces);
        }

        Terminal::flush();
    }

    fn handle_keypress(&mut self) {
        let pressed_key = Terminal::read_key().unwrap();

        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up => self.cursor.mut_move(0, -1),
            Key::Down => self.cursor.mut_move(0, 1),
            Key::Right => self.cursor.mut_move(1, 0),
            Key::Left => self.cursor.mut_move(-1, 0),
            Key::Char(' ') => self
                .board
                .open_tile(&self.cursor.position)
                .unwrap_or_else(|()| self.should_quit = true),
            Key::Char('f') => self.board.tile_at(&self.cursor.position).flag(),
            _ => {}
        }
    }

    pub fn read_key() -> Result<Key, io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn clear_after_cursor() {
        print!("{}", termion::clear::AfterCursor);
    }

    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub fn cursor_goto(x: u16, y: u16) {
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub fn bg_color_str(val: &str, color: impl termion::color::Color) -> String {
        format!(
            "{}{}{}",
            termion::color::Bg(color),
            val,
            termion::color::Bg(termion::color::Reset)
        )
    }

    pub fn flush() {
        io::stdout().flush().unwrap();
    }
}
