use mineswpr::{board::Board, terminal::Terminal};

fn main() {
    let board = Board::new(3, 3, 2);
    let terminal = Terminal::new(board);
    terminal.run()
}
