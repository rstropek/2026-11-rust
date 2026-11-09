mod tictactoe;

use std::{
    io::{self, Write},
    str::FromStr,
};
use tictactoe::{Board, BoardPosition, GameStatus, PlaceStoneError, Player};
use BoardPosition::*;

pub enum ParseBoardPositionError {
    FormatNotSupported,
    OutOfRange,
}

impl FromStr for BoardPosition {
    type Err = ParseBoardPositionError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let position = if let Ok(number) = input.parse::<i32>() {
            // Num-pad style: 1 is left lower, 9 is right upper
            match number {
                1 => C1,
                2 => C2,
                3 => C3,
                4 => B1,
                5 => B2,
                6 => B3,
                7 => A1,
                8 => A2,
                9 => A3,
                _ => return Err(ParseBoardPositionError::OutOfRange),
            }
        } else if let [row @ b'A'..=b'Z', col @ b'1'..=b'9'] = input.as_bytes() {
            // Chess style: A-C for rows, 1-3 for cols
            match (row, col) {
                (b'A', b'1') => A1,
                (b'A', b'2') => A2,
                (b'A', b'3') => A3,
                (b'B', b'1') => B1,
                (b'B', b'2') => B2,
                (b'B', b'3') => B3,
                (b'C', b'1') => C1,
                (b'C', b'2') => C2,
                (b'C', b'3') => C3,
                _ => return Err(ParseBoardPositionError::OutOfRange),
            }
        } else {
            return Err(ParseBoardPositionError::FormatNotSupported);
        };

        Ok(position)
    }
}

fn main() {
    let mut board = Board::new();
    let mut input = String::new();

    loop {
        print!("Your next move (1-9 or A1-C3): ");
        io::stdout().flush().expect("Failed to write to console");

        input.clear();
        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from console");

        // Zero bytes means the input was closed (e.g. Ctrl+D)
        if bytes_read == 0 {
            println!();
            break;
        }

        let pos: BoardPosition = match input.trim().parse() {
            Ok(pos) => pos,
            Err(ParseBoardPositionError::FormatNotSupported) => {
                println!("Please enter a number (1-9) or a position like B2.");
                continue;
            }
            Err(ParseBoardPositionError::OutOfRange) => {
                println!("This position is not on the board.");
                continue;
            }
        };

        let result = board.place_stone(pos);

        match result {
            Err(PlaceStoneError::AlreadyOccupied) => println!("This field is already occupied."),
            Err(PlaceStoneError::GameAlreadyOver) => println!("The game is already over."),
            Ok(_) => {}
        }

        println!("{board:?}");

        match result {
            Ok(GameStatus::Winner(Player::X)) => {
                println!("Player X wins!");
                break;
            }
            Ok(GameStatus::Winner(Player::O)) => {
                println!("Player O wins!");
                break;
            }
            Ok(GameStatus::Draw) => {
                println!("It's a draw!");
                break;
            }
            Ok(GameStatus::OngoingGame) | Err(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parses_to(input: &str, expected: BoardPosition) {
        let Ok(parsed) = input.parse::<BoardPosition>() else {
            panic!("{input:?} should parse");
        };

        assert_eq!(parsed, expected, "input: {input:?}");
    }

    fn assert_out_of_range(input: &str) {
        assert!(
            matches!(
                input.parse::<BoardPosition>(),
                Err(ParseBoardPositionError::OutOfRange)
            ),
            "input: {input:?}"
        );
    }

    fn assert_format_not_supported(input: &str) {
        assert!(
            matches!(
                input.parse::<BoardPosition>(),
                Err(ParseBoardPositionError::FormatNotSupported)
            ),
            "input: {input:?}"
        );
    }

    #[test]
    fn parses_all_num_pad_positions() {
        let cases = [
            ("1", C1),
            ("2", C2),
            ("3", C3),
            ("4", B1),
            ("5", B2),
            ("6", B3),
            ("7", A1),
            ("8", A2),
            ("9", A3),
        ];

        for (input, expected) in cases {
            assert_parses_to(input, expected);
        }
    }

    #[test]
    fn parses_all_chess_positions() {
        let cases = [
            ("A1", A1),
            ("A2", A2),
            ("A3", A3),
            ("B1", B1),
            ("B2", B2),
            ("B3", B3),
            ("C1", C1),
            ("C2", C2),
            ("C3", C3),
        ];

        for (input, expected) in cases {
            assert_parses_to(input, expected);
        }
    }

    #[test]
    fn rejects_numbers_out_of_range() {
        for input in ["0", "10", "123", "-1"] {
            assert_out_of_range(input);
        }
    }

    #[test]
    fn rejects_chess_row_out_of_range() {
        for input in ["D1", "Z3"] {
            assert_out_of_range(input);
        }
    }

    #[test]
    fn rejects_chess_col_out_of_range() {
        for input in ["A4", "C9"] {
            assert_out_of_range(input);
        }
    }

    #[test]
    fn rejects_unsupported_formats() {
        for input in [
            "",
            "x",
            "A",
            "a1",
            "A0",
            "A12",
            "1A",
            "AA",
            "ä1",
            "5\n",
            "99999999999",
        ] {
            assert_format_not_supported(input);
        }
    }
}
