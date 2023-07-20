use mineswpr::{board::Board, terminal::Terminal};

fn main() {
    let board = Board::new(10, 10, 8);
    let mut terminal = Terminal::new(board);
    terminal.run()
}
