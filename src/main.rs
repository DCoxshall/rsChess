mod engine;

use engine::Board;

fn main() {
    let my_board =
        Board::parse_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());

    println!("{}", my_board);
}
