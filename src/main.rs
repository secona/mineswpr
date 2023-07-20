use mineswpr::{board::Board, terminal::Terminal};

fn main() {
    let board = Board::new(10, 10, 2);
    let mut terminal = Terminal::new(board);
    terminal.run()
}
