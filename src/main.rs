mod engine;

use engine::Board;

fn main() {
    let my_board = Board {
        white_pawns: 0,
        white_rooks: 0,
        white_knights: 0,
        white_bishops: 0,
        white_queens: 0,
        white_king: 0,
        black_pawns: 0,
        black_rooks: 0,
        black_knights: 0,
        black_bishops: 0,
        black_queens: 0,
        black_king: 0,
    };

    println!("{}", my_board);
}
