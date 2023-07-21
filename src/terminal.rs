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
    locked: bool,
}

impl From<&Board> for Cursor {
    fn from(value: &Board) -> Self {
        Self {
            position: Point::default(),
            max: Point::new(value.width, value.height),
            locked: false,
        }
    }
}

impl Cursor {
    pub fn mut_move(&mut self, x: i32, y: i32) {
        if !self.locked {
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

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn unlock(&mut self) {
        self.locked = false;
    }
}

enum GameState {
    Running,
    Stopped,
}

enum GameResult {
    Win,
    Lose,
    Pending,
}

pub struct Terminal {
    board: Board,
    state: GameState,
    result: GameResult,
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
            state: GameState::Running,
            result: GameResult::Pending,
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

            if let GameState::Stopped = self.state {
                break;
            }

            self.handle_keypress();
            self.check_for_win();
        }

        Terminal::cursor_show();
    }

    fn refresh_screen(&self) {
        Terminal::cursor_goto(1, 1);

        if let GameState::Stopped = self.state {
            Terminal::cursor_goto(1, 1);
            Terminal::clear_after_cursor();
            println!("Goodbye. Thank you for playing.");

            return;
        }

        self.draw_board();
        self.draw_message();
        self.draw_help_message();

        Terminal::flush();
    }

    fn draw_board(&self) {
        for (y, row) in self.board.tiles.iter().enumerate() {
            let mut result: Vec<String> = Vec::new();
            for (x, tile) in row.iter().enumerate() {
                let mut tile = self.color_tile(tile);

                if self.cursor.position == Point::new(x, y) {
                    tile = Terminal::bg_color_str(&tile, termion::color::White);
                }

                result.push(tile);
            }

            println!(
                "{}{}\r",
                result.join(" "),
                " ".repeat(self.width - (self.board.width * 2 - 1))
            );
        }
    }

    fn draw_message(&self) {
        let message = match self.result {
            GameResult::Win => {
                Terminal::fg_color_str("Congratulations! You Won!\r", termion::color::LightGreen)
            }
            GameResult::Lose => {
                Terminal::fg_color_str("Game Over! You Lost!\r", termion::color::LightRed)
            }
            _ => " ".repeat(self.width),
        };

        println!("\n\r{}\n\r\r", message);
    }

    fn draw_help_message(&self) {
        let key_color = termion::color::LightBlue;

        println!(
            "Press {} to move cursor.\r",
            Terminal::fg_color_str("arrow keys", key_color)
        );
        println!(
            "Press {} to open a tile.\r",
            Terminal::fg_color_str("space", key_color)
        );
        println!(
            "Press {} to flag a tile.\r",
            Terminal::fg_color_str("F", key_color)
        );
        println!(
            "Press {} to restart the game.\r",
            Terminal::fg_color_str("ctrl + r", key_color)
        );
        println!(
            "Press {} to quit the game.\r",
            Terminal::fg_color_str("ctrl + q", key_color)
        );
    }

    fn color_tile(&self, tile: &tile::Tile) -> String {
        match tile.state() {
            tile::State::Opened => match tile.value() {
                tile::Value::Number(n) => {
                    let value = &n.to_string();
                    match n {
                        0 => String::from(" "), // transparent color
                        1 => Terminal::fg_color_str(value, termion::color::LightBlue),
                        2 => Terminal::fg_color_str(value, termion::color::Green),
                        3 => Terminal::fg_color_str(value, termion::color::Red),
                        4 => Terminal::fg_color_str(value, termion::color::Blue),
                        5 => Terminal::fg_color_str(value, termion::color::Magenta),
                        6 => Terminal::fg_color_str(value, termion::color::Cyan),
                        7 => Terminal::fg_color_str(value, termion::color::LightCyan),
                        8 => Terminal::fg_color_str(value, termion::color::Yellow),
                        _ => value.to_owned(),
                    }
                }
                tile::Value::Mine => Terminal::fg_color_str("X", termion::color::Red),
            },
            tile::State::Closed => Terminal::fg_color_str("?", termion::color::LightBlack),
            tile::State::Flagged => Terminal::fg_color_str("F", termion::color::Rgb(200, 0, 200)),
        }
    }

    fn handle_keypress(&mut self) {
        let pressed_key = Terminal::read_key().unwrap();

        match pressed_key {
            Key::Ctrl('q') => self.quit(),
            Key::Ctrl('r') => self.restart(),
            Key::Up => self.cursor.mut_move(0, -1),
            Key::Down => self.cursor.mut_move(0, 1),
            Key::Right => self.cursor.mut_move(1, 0),
            Key::Left => self.cursor.mut_move(-1, 0),
            Key::Char(' ') => self
                .board
                .open_tile(&self.cursor.position)
                .unwrap_or_else(|()| self.game_over()),
            Key::Char('f') => self.board.tile_at(&self.cursor.position).flag(),
            _ => {}
        }
    }

    fn check_for_win(&mut self) {
        if self.board.has_won() {
            self.result = GameResult::Win;
            self.cursor.lock();
        }
    }

    fn quit(&mut self) {
        self.state = GameState::Stopped;
    }

    fn game_over(&mut self) {
        self.result = GameResult::Lose;
        self.cursor.lock();
    }

    fn restart(&mut self) {
        self.state = GameState::Running;
        self.result = GameResult::Pending;
        self.board.restart();
        self.cursor.unlock();
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

    pub fn fg_color_str(val: &str, color: impl termion::color::Color) -> String {
        format!(
            "{}{}{}",
            termion::color::Fg(color),
            val,
            termion::color::Fg(termion::color::Reset)
        )
    }

    pub fn flush() {
        io::stdout().flush().unwrap();
    }
}
