use std::fmt::Display;

pub struct Board {
    pub white_pawns: u64,
    pub white_rooks: u64,
    pub white_knights: u64,
    pub white_bishops: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_rooks: u64,
    pub black_knights: u64,
    pub black_bishops: u64,
    pub black_queens: u64,
    pub black_king: u64,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_repr: [char; 64] = ['.'; 64];

        // There has to be a better way of populating the array than this
        board_repr = Board::populate_array(board_repr, 'P', &self.white_pawns);
        board_repr = Board::populate_array(board_repr, 'R', &self.white_rooks);
        board_repr = Board::populate_array(board_repr, 'N', &self.white_knights);
        board_repr = Board::populate_array(board_repr, 'B', &self.white_bishops);
        board_repr = Board::populate_array(board_repr, 'Q', &self.white_queens);
        board_repr = Board::populate_array(board_repr, 'K', &self.white_king);
        board_repr = Board::populate_array(board_repr, 'p', &self.black_pawns);
        board_repr = Board::populate_array(board_repr, 'r', &self.black_rooks);
        board_repr = Board::populate_array(board_repr, 'n', &self.black_knights);
        board_repr = Board::populate_array(board_repr, 'b', &self.black_bishops);
        board_repr = Board::populate_array(board_repr, 'q', &self.black_queens);
        board_repr = Board::populate_array(board_repr, 'k', &self.black_king);

        let mut string_board: String = "".to_string();

        for i in 0..8 {
            for j in 0..8 {
                string_board.push(board_repr[i * 8 + j]);
                string_board.push(' ');
            }
            string_board.push('\n');
        }

        write!(f, "{}", string_board)
    }
}

impl Board {
    fn populate_array(mut arr: [char; 64], piece: char, bitboard: &u64) -> [char; 64] {
        for i in 0..64 {
            if (bitboard >> i) & 1 == 1 {
                arr[i] = piece;
            }
        }

        arr
    }
}
