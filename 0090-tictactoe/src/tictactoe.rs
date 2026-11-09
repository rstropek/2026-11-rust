use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Copy)]
pub enum GameStatus {
    Winner(Player),
    Draw,
    OngoingGame,
}

// All fields of the board: A-C for rows (top to bottom), 1-3 for cols (left to right).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BoardPosition {
    A1,
    A2,
    A3,
    B1,
    B2,
    B3,
    C1,
    C2,
    C3,
}

impl BoardPosition {
    // The (row, col) index of this position in `Board::fields`.
    fn row_col(self) -> (usize, usize) {
        match self {
            Self::A1 => (0, 0),
            Self::A2 => (0, 1),
            Self::A3 => (0, 2),
            Self::B1 => (1, 0),
            Self::B2 => (1, 1),
            Self::B3 => (1, 2),
            Self::C1 => (2, 0),
            Self::C2 => (2, 1),
            Self::C3 => (2, 2),
        }
    }
}

pub enum PlaceStoneError {
    AlreadyOccupied,
    GameAlreadyOver,
}

// All lines that win the game, as (row, col) positions.
const WINNING_LINES: [[(usize, usize); 3]; 8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],
    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],
    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

pub struct Board {
    fields: [[Option<Player>; 3]; 3],
    next_player: Player,
    game_state: GameStatus,
}

impl Board {
    pub fn new() -> Self {
        Self {
            fields: [[None; 3]; 3],
            next_player: Player::X,
            game_state: GameStatus::OngoingGame,
        }
    }

    pub fn place_stone(&mut self, pos: BoardPosition) -> Result<GameStatus, PlaceStoneError> {
        if !matches!(self.game_state, GameStatus::OngoingGame) {
            return Err(PlaceStoneError::GameAlreadyOver);
        }

        let (row, col) = pos.row_col();
        let field = &mut self.fields[row][col];
        if field.is_some() {
            return Err(PlaceStoneError::AlreadyOccupied);
        }

        let player = self.next_player;
        *field = Some(player);

        self.game_state = if self.has_won(player) {
            GameStatus::Winner(player)
        } else if self.is_full() {
            GameStatus::Draw
        } else {
            GameStatus::OngoingGame
        };

        self.next_player = match player {
            Player::X => Player::O,
            Player::O => Player::X,
        };

        Ok(self.game_state)
    }

    fn has_won(&self, player: Player) -> bool {
        let stone = Some(player);

        for [(r1, c1), (r2, c2), (r3, c3)] in WINNING_LINES {
            if self.fields[r1][c1] == stone
                && self.fields[r2][c2] == stone
                && self.fields[r3][c3] == stone
            {
                return true;
            }
        }

        false
    }

    fn is_full(&self) -> bool {
        for row in &self.fields {
            for field in row {
                if field.is_none() {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (row_index, row) in self.fields.iter().enumerate() {
            if row_index > 0 {
                f.write_str("\n---+---+---\n")?;
            }

            for (col_index, field) in row.iter().enumerate() {
                if col_index > 0 {
                    f.write_str("|")?;
                }

                f.write_str(match field {
                    Some(Player::X) => " X ",
                    Some(Player::O) => " O ",
                    None => "   ",
                })?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BoardPosition::*;
    use super::*;

    #[test]
    fn row_col_maps_every_position() {
        let cases = [
            (A1, (0, 0)),
            (A2, (0, 1)),
            (A3, (0, 2)),
            (B1, (1, 0)),
            (B2, (1, 1)),
            (B3, (1, 2)),
            (C1, (2, 0)),
            (C2, (2, 1)),
            (C3, (2, 2)),
        ];

        for (position, expected) in cases {
            assert_eq!(position.row_col(), expected);
        }
    }

    // Plays the given moves. Every move must leave the game ongoing.
    fn play(board: &mut Board, moves: &[BoardPosition]) {
        for &position in moves {
            assert!(matches!(
                board.place_stone(position),
                Ok(GameStatus::OngoingGame)
            ));
        }
    }

    fn render(board: &Board) -> String {
        format!("{board:?}")
    }

    fn board_won_by_x() -> Board {
        let mut board = Board::new();
        play(&mut board, &[A1, B1, A2, B2]);
        assert!(matches!(
            board.place_stone(A3),
            Ok(GameStatus::Winner(Player::X))
        ));
        board
    }

    fn drawn_board() -> Board {
        let mut board = Board::new();
        play(&mut board, &[A1, A2, A3, B2, C2, C1, B1, B3]);
        assert!(matches!(board.place_stone(C3), Ok(GameStatus::Draw)));
        board
    }

    #[test]
    fn new_board_is_empty() {
        let expected = "   |   |   \n---+---+---\n   |   |   \n---+---+---\n   |   |   ";
        assert_eq!(render(&Board::new()), expected);
    }

    #[test]
    fn place_stone_alternates_players_starting_with_x() {
        let mut board = Board::new();
        play(&mut board, &[A1, A2, A3]);

        let expected = " X | O | X \n---+---+---\n   |   |   \n---+---+---\n   |   |   ";
        assert_eq!(render(&board), expected);
    }

    #[test]
    fn place_stone_rejects_occupied_field_and_keeps_turn() {
        let mut board = Board::new();
        play(&mut board, &[A1]);

        assert!(matches!(
            board.place_stone(A1),
            Err(PlaceStoneError::AlreadyOccupied)
        ));

        play(&mut board, &[A2]);
        let expected = " X | O |   \n---+---+---\n   |   |   \n---+---+---\n   |   |   ";
        assert_eq!(render(&board), expected);
    }

    #[test]
    fn place_stone_reports_winner_x() {
        board_won_by_x();
    }

    #[test]
    fn place_stone_reports_winner_o() {
        let mut board = Board::new();
        play(&mut board, &[A1, A3, A2, B2, C3]);

        assert!(matches!(
            board.place_stone(C1),
            Ok(GameStatus::Winner(Player::O))
        ));
    }

    #[test]
    fn place_stone_prefers_winner_over_draw_on_last_field() {
        let mut board = Board::new();
        play(&mut board, &[A1, A2, A3, B1, C1, B3, C3, C2]);

        assert!(matches!(
            board.place_stone(B2),
            Ok(GameStatus::Winner(Player::X))
        ));
    }

    #[test]
    fn place_stone_reports_draw() {
        drawn_board();
    }

    #[test]
    fn place_stone_rejects_free_field_after_win() {
        let mut board = board_won_by_x();
        assert!(matches!(
            board.place_stone(C3),
            Err(PlaceStoneError::GameAlreadyOver)
        ));
    }

    #[test]
    fn place_stone_reports_game_over_before_occupied_field() {
        let mut board = board_won_by_x();
        assert!(matches!(
            board.place_stone(A1),
            Err(PlaceStoneError::GameAlreadyOver)
        ));
    }

    #[test]
    fn place_stone_rejects_move_after_draw() {
        let mut board = drawn_board();
        assert!(matches!(
            board.place_stone(A1),
            Err(PlaceStoneError::GameAlreadyOver)
        ));
    }

    #[test]
    fn has_won_detects_every_line() {
        // Written as (col, row), independently of WINNING_LINES.
        let lines = [
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(2, 0), (1, 1), (0, 2)],
        ];

        for line in lines {
            let mut board = Board::new();
            for (col, row) in line {
                board.fields[row][col] = Some(Player::O);
            }

            assert!(board.has_won(Player::O));
            assert!(!board.has_won(Player::X));
        }
    }

    #[test]
    fn debug_renders_board_from_design() {
        let mut board = Board::new();
        play(&mut board, &[A2, A3, B1, B2, C2, C3]);

        let expected = "   | X | O \n---+---+---\n X | O |   \n---+---+---\n   | X | O ";
        assert_eq!(render(&board), expected);
    }
}
