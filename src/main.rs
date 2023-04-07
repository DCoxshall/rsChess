mod engine;

use engine::Board;

fn main() {
    let fen = "8/p3k1N1/8/6p1/P7/1P2n3/5P1P/2r3K1 w - e4".to_string();
    let my_board = Board::parse_fen(&fen);
    match my_board {
        Ok(board) => println!("{}", board),
        Err(e) => println!("{}", e),
    }
}
