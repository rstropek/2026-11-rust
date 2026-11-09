mod tictactoe;

use std::io::{self, Write};
use tictactoe::{Board, BoardIndex, GameStatus, PlaceStoneError, Player};

enum ParseBoardIndexError {
    FormatNotSupported,
    OutOfRange,
}

fn parse_board_index(input: &str) -> Result<BoardIndex, ParseBoardIndexError> {
    let (col, row) = if let Ok(number) = input.parse::<i32>() {
        // Num-pad style: 1 is left lower, 9 is right upper
        if !(1..=9).contains(&number) {
            return Err(ParseBoardIndexError::OutOfRange);
        }

        let zero_based = number as u8 - 1;
        (zero_based % 3, 2 - zero_based / 3)
    } else if let [row @ b'A'..=b'Z', col @ b'1'..=b'9'] = input.as_bytes() {
        // Chess style: A-C for rows, 1-3 for cols
        (col - b'1', row - b'A')
    } else {
        return Err(ParseBoardIndexError::FormatNotSupported);
    };

    BoardIndex::new(col, row).ok_or(ParseBoardIndexError::OutOfRange)
}

fn main() {
    let mut board = Board::new();
    let mut input = String::new();
    let mut output = String::new();

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

        let pos = match parse_board_index(input.trim()) {
            Ok(pos) => pos,
            Err(ParseBoardIndexError::FormatNotSupported) => {
                println!("Please enter a number (1-9) or a position like B2.");
                continue;
            }
            Err(ParseBoardIndexError::OutOfRange) => {
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

        output.clear();
        board.to_string(&mut output);
        println!("{output}");

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

    fn assert_parses_to(input: &str, col: u8, row: u8) {
        let Ok(parsed) = parse_board_index(input) else {
            panic!("{input:?} should parse");
        };

        assert_eq!(
            (parsed.get_col(), parsed.get_row()),
            (col, row),
            "input: {input:?}"
        );
    }

    fn assert_out_of_range(input: &str) {
        assert!(
            matches!(
                parse_board_index(input),
                Err(ParseBoardIndexError::OutOfRange)
            ),
            "input: {input:?}"
        );
    }

    fn assert_format_not_supported(input: &str) {
        assert!(
            matches!(
                parse_board_index(input),
                Err(ParseBoardIndexError::FormatNotSupported)
            ),
            "input: {input:?}"
        );
    }

    #[test]
    fn parses_all_num_pad_positions() {
        let cases = [
            ("1", 0, 2),
            ("2", 1, 2),
            ("3", 2, 2),
            ("4", 0, 1),
            ("5", 1, 1),
            ("6", 2, 1),
            ("7", 0, 0),
            ("8", 1, 0),
            ("9", 2, 0),
        ];

        for (input, col, row) in cases {
            assert_parses_to(input, col, row);
        }
    }

    #[test]
    fn parses_all_chess_positions() {
        let cases = [
            ("A1", 0, 0),
            ("A2", 1, 0),
            ("A3", 2, 0),
            ("B1", 0, 1),
            ("B2", 1, 1),
            ("B3", 2, 1),
            ("C1", 0, 2),
            ("C2", 1, 2),
            ("C3", 2, 2),
        ];

        for (input, col, row) in cases {
            assert_parses_to(input, col, row);
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
