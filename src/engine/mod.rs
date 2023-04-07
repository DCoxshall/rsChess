use std::{collections::HashMap, fmt::Display};

use regex::Regex;

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

            en_passant_target: match fields.get(3) {
                None => return Err(String::from("Invalid FEN: No en passant target found.")),
                Some(square) => match Board::convert_square_to_bitboard(square) {
                    Ok(bitboard) => bitboard,
                    Err(err) => {
                        let mut error_string: String =
                            String::from("Invalid FEN: Invalid en passant target: ");
                        error_string.push_str(err);
                        return Err(error_string);
                    }
                },
            },
            half_move_clock: fields[4].to_string().parse::<i32>().unwrap(),
            full_move_clock: fields[5].to_string().parse::<i32>().unwrap(),
        };

        let mut pos = 0;
        for c in fields[0].chars() {
            if "PRNBQKprnbqk".contains(c) {
                new_board.bitboards.insert(
                    c,
                    new_board.bitboards.get(&c).unwrap() | 9223372036854775808 >> pos,
                );
            } else if "12345678".contains(c) {
                pos += c.to_digit(10).unwrap();
            }

            if String::from("PRNBQKprnbqk").contains(c) {
                pos += 1;
            }
        }

        Ok(new_board)
    }

    fn convert_square_to_bitboard(square: &str) -> Result<u64, &str> {
        let alphabet = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        let square_file = match square.chars().nth(0) {
            Some(ch) if "abcdefgh".contains(ch) => ch,
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
        let file_number: i32 = alphabet.iter().position(|&x| x == square_file).unwrap() as i32;
        let rank_number = square_rank.to_string().parse::<i32>().unwrap() - 1;
        return Ok(9223372036854775808 >> (file_number * 8 + rank_number));
        // 9223372036854775808 is the number which represents a 1 followed by 63 zeroes.
    }
}
