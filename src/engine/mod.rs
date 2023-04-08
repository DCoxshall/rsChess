use std::{collections::HashMap, fmt::Display};

use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    // Bitboards
    pub bitboards: HashMap<char, u64>,
    // Hash map is accessed with the character of the piece whose bitboard you
    // want to see

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

        for (piece, bitmap) in &self.bitboards {
            board_repr = Board::populate_array(board_repr, *piece, bitmap);
        }

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

    pub fn parse_fen(fen_string: &String) -> Result<Board, String> {
        let fields: Vec<&str> = fen_string.split_whitespace().collect();
        // This regex would also in theory match an empty string, but
        let castling_regex = Regex::new("[^K?Q?k?q?$]|[^-$]").unwrap();
        let en_passant_regex = Regex::new("^([a-h][1-8]$)|^-$").unwrap();

        if fields.len() != 6 {
            return Err(String::from("Invalid FEN: One or more fields are missing."));
        }

        let mut new_board = Board {
            bitboards: HashMap::from([
                ('P', 0),
                ('R', 0),
                ('N', 0),
                ('B', 0),
                ('Q', 0),
                ('K', 0),
                ('p', 0),
                ('r', 0),
                ('n', 0),
                ('b', 0),
                ('q', 0),
                ('k', 0),
            ]),

            // Unwraps are justified because each field is guaranteed to have
            // one or more characters.
            turn: match fields[1].chars().nth(0).unwrap() {
                'w' | 'b' => fields[1].chars().nth(0).unwrap(),
                _ => return Err(String::from("Invalid turn field.")),
            },

            castling_rights: match fields[2] {
                castling if castling_regex.is_match(castling) => castling.to_string(),
                _ => return Err(String::from("Invalid castling field.")),
            },

            en_passant_target: match fields[3] {
                en_passant_target if en_passant_regex.is_match(en_passant_target) => {
                    Board::convert_square_to_bitboard(en_passant_target).unwrap()
                }
                _ => return Err(String::from("Invalid en passant field.")),
            },

            half_move_clock: fields[4].to_string().parse::<i32>().unwrap(),
            full_move_clock: fields[5].to_string().parse::<i32>().unwrap(),
        };

        let mut pos = 0;
        for c in fields[0].chars() {
            if "PRNBQKprnbqk".contains(c) {
                new_board.bitboards.insert(
                    c,
                    new_board.bitboards.get(&c).unwrap() | 1 << 63 - pos,
                );
                pos += 1;
            } else if "12345678".contains(c) {
                pos += c.to_digit(10).unwrap();
            }
        }

        Ok(new_board)
    }

    fn convert_square_to_bitboard(square: &str) -> Result<u64, &str> {
        let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let square_file = match square.chars().nth(0) {
            Some(ch) if "abcdefgh-".contains(ch) => ch,
            Some(_ch) => return Err("No file provided."),
            None => return Err("No en passant square provided."),
        };

        if square_file == '-' {
            // No en passant square: return empty bitboard
            return Ok(0);
        }
        let square_rank = match square.chars().nth(1) {
            Some(ch) if "12345678".contains(ch) => ch,
            None => return Err("No rank provided."),
            _ => return Err("Invalid rank provided."),
        };
        let file_number: i32 = 7 - alphabet.iter().position(|&x| x == square_file).unwrap() as i32;
        let rank_number = square_rank.to_string().parse::<i32>().unwrap() - 1;
        Ok(1 << rank_number * 8 + file_number)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_fen {
        ($name:tt: $fen:literal, $($field:tt = $value:expr),* $(,)?) => {
            paste::paste! {
                $(
                    #[test]
                    fn [<$name _ $field>]() {
                        let Board { $field, .. } = Board::parse_fen(&$fen.to_string()).unwrap();
                        assert_eq!($field, $value);
                    }
                )*
            }
        }
    }

    test_fen!(start_position: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        turn = 'w',
        castling_rights = "KQkq",
        en_passant_target = 0,
        half_move_clock = 0,
        full_move_clock = 1,
    );

    test_fen!(en_passant: "rnbqkbnr/1ppp1ppp/p7/3Pp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 3",
        turn = 'w',
        castling_rights = "KQkq",
        en_passant_target = 1 << (5 * 8) + 3,
        half_move_clock = 0,
        full_move_clock = 3,
    );

    test_fen!(no_castle: "rnbq1bnr/1pppkppp/p7/3Pp3/8/3Q4/PPP1PPPP/RNBK1BNR b - - 3 4",
        turn = 'b',
        castling_rights = "-",
        en_passant_target = 0,
        half_move_clock = 3,
        full_move_clock = 4
    );
}
