use std::fmt::Display;

pub struct Board {
    // Bitboards
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

    // FEN information
    pub turn: char,
    pub castling_rights: String,
    pub en_passant_target: u64,
    pub half_move_clock: i32,
    pub full_move_clock: i32,
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

        for i in (0..8).rev() {
            for j in (0..8).rev() {
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
            // Bit fuckery #1: perform right shift 64 times and AND against 1 to determine if there is a piece on this square.
            if (bitboard >> i) & 1 == 1 {
                arr[i] = piece;
            }
        }

        arr
    }

    pub fn parse_fen(fen_string: &String) -> Board {
        let fields: Vec<&str> = fen_string.split_whitespace().collect();
        let mut new_board = Board {
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

            turn: fields[1].chars().nth(0).unwrap(),
            castling_rights: fields[2].to_string(),
            en_passant_target: Board::convert_square_to_bitboard(&fields[3]),
            half_move_clock: fields[4].to_string().parse::<i32>().unwrap(),
            full_move_clock: fields[5].to_string().parse::<i32>().unwrap(),
        };

        let mut pos = 0;
        for c in fields[0].chars() {
            match c {
                'P' => new_board.white_pawns |= 9223372036854775808 >> pos,
                'R' => new_board.white_rooks |= 9223372036854775808 >> pos,
                'N' => new_board.white_knights |= 9223372036854775808 >> pos,
                'B' => new_board.white_bishops |= 9223372036854775808 >> pos,
                'Q' => new_board.white_queens |= 9223372036854775808 >> pos,
                'K' => new_board.white_king |= 9223372036854775808 >> pos,
                'p' => new_board.black_pawns |= 9223372036854775808 >> pos,
                'r' => new_board.black_rooks |= 9223372036854775808 >> pos,
                'n' => new_board.black_knights |= 9223372036854775808 >> pos,
                'b' => new_board.black_bishops |= 9223372036854775808 >> pos,
                'q' => new_board.black_queens |= 9223372036854775808 >> pos,
                'k' => new_board.black_king |= 9223372036854775808 >> pos,
                '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => pos += c.to_digit(10).unwrap(),
                '/' => {}
                _ => panic!(),
            }
            if String::from("PRNBQKprnbqk").contains(c) {
                pos += 1;
            }
        }

        new_board
    }

    fn convert_square_to_bitboard(square: &str) -> u64 {
        if square.len() == 1 {
            // square could only be '-', which means there is no valid en passant target
            return 0;
        }
        let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let square_file = square.chars().nth(0).unwrap();
        let square_rank = square.chars().nth(1).unwrap();
        let file_number: i32 = alphabet.iter().position(|&x| x == square_file).unwrap() as i32;
        let rank_number = square_rank.to_string().parse::<i32>().unwrap() - 1;
        return 9223372036854775808 >> (file_number * 8 + rank_number);
        // 9223372036854775808 is the number which represents a 1 followed by 63 zeroes.
    }
}
