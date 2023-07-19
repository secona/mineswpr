use std::io::{self, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

use crate::board::Board;
use crate::tile;

pub struct Terminal {
    board: Board,
    should_quit: bool,
    width: usize,
    height: usize,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn new(board: Board) -> Self {
        let term_size = termion::terminal_size().unwrap();

        Self {
            board,
            width: term_size.0 as usize,
            height: term_size.1 as usize,
            should_quit: false,
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

        for row in &self.board.tiles {
            let mut result: Vec<String> = Vec::new();
            for tile in row {
                match tile.value() {
                    tile::Value::Number(n) => result.push(n.to_string()),
                    tile::Value::Mine => result.push("X".to_string()),
                }
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

    pub fn flush() {
        io::stdout().flush().unwrap();
    }
}
