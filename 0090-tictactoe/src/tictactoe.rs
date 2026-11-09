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
#[derive(Clone, Copy)]
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

pub struct BoardIndex {
    col: u8,
    row: u8,
}

impl BoardIndex {
    pub fn new(position: BoardPosition) -> Self {
        let (col, row) = match position {
            BoardPosition::A1 => (0, 0),
            BoardPosition::A2 => (1, 0),
            BoardPosition::A3 => (2, 0),
            BoardPosition::B1 => (0, 1),
            BoardPosition::B2 => (1, 1),
            BoardPosition::B3 => (2, 1),
            BoardPosition::C1 => (0, 2),
            BoardPosition::C2 => (1, 2),
            BoardPosition::C3 => (2, 2),
        };

        Self { col, row }
    }

    #[cfg(test)]
    pub fn get_col(&self) -> u8 {
        self.col
    }

    #[cfg(test)]
    pub fn get_row(&self) -> u8 {
        self.row
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

    pub fn place_stone(&mut self, pos: BoardIndex) -> Result<GameStatus, PlaceStoneError> {
        if !matches!(self.game_state, GameStatus::OngoingGame) {
            return Err(PlaceStoneError::GameAlreadyOver);
        }

        let field = &mut self.fields[pos.row as usize][pos.col as usize];
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

    pub fn to_string(&self, output: &mut String) {
        for (row_index, row) in self.fields.iter().enumerate() {
            if row_index > 0 {
                output.push_str("\n---+---+---\n");
            }

            for (col_index, field) in row.iter().enumerate() {
                if col_index > 0 {
                    output.push('|');
                }

                output.push_str(match field {
                    Some(Player::X) => " X ",
                    Some(Player::O) => " O ",
                    None => "   ",
                });
            }
        }
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

#[cfg(test)]
mod tests {
    use super::BoardPosition::*;
    use super::*;

    #[test]
    fn new_maps_every_position_to_col_and_row() {
        let cases = [
            (A1, 0, 0),
            (A2, 1, 0),
            (A3, 2, 0),
            (B1, 0, 1),
            (B2, 1, 1),
            (B3, 2, 1),
            (C1, 0, 2),
            (C2, 1, 2),
            (C3, 2, 2),
        ];

        for (position, col, row) in cases {
            let index = BoardIndex::new(position);
            assert_eq!((index.col, index.row), (col, row));
        }
    }

    // Plays the given moves. Every move must leave the game ongoing.
    fn play(board: &mut Board, moves: &[BoardPosition]) {
        for &position in moves {
            assert!(matches!(
                board.place_stone(BoardIndex::new(position)),
                Ok(GameStatus::OngoingGame)
            ));
        }
    }

    fn render(board: &Board) -> String {
        let mut output = String::new();
        board.to_string(&mut output);
        output
    }

    fn board_won_by_x() -> Board {
        let mut board = Board::new();
        play(&mut board, &[A1, B1, A2, B2]);
        assert!(matches!(
            board.place_stone(BoardIndex::new(A3)),
            Ok(GameStatus::Winner(Player::X))
        ));
        board
    }

    fn drawn_board() -> Board {
        let mut board = Board::new();
        play(&mut board, &[A1, A2, A3, B2, C2, C1, B1, B3]);
        assert!(matches!(
            board.place_stone(BoardIndex::new(C3)),
            Ok(GameStatus::Draw)
        ));
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
            board.place_stone(BoardIndex::new(A1)),
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
            board.place_stone(BoardIndex::new(C1)),
            Ok(GameStatus::Winner(Player::O))
        ));
    }

    #[test]
    fn place_stone_prefers_winner_over_draw_on_last_field() {
        let mut board = Board::new();
        play(&mut board, &[A1, A2, A3, B1, C1, B3, C3, C2]);

        assert!(matches!(
            board.place_stone(BoardIndex::new(B2)),
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
            board.place_stone(BoardIndex::new(C3)),
            Err(PlaceStoneError::GameAlreadyOver)
        ));
    }

    #[test]
    fn place_stone_reports_game_over_before_occupied_field() {
        let mut board = board_won_by_x();
        assert!(matches!(
            board.place_stone(BoardIndex::new(A1)),
            Err(PlaceStoneError::GameAlreadyOver)
        ));
    }

    #[test]
    fn place_stone_rejects_move_after_draw() {
        let mut board = drawn_board();
        assert!(matches!(
            board.place_stone(BoardIndex::new(A1)),
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
    fn to_string_renders_board_from_design() {
        let mut board = Board::new();
        play(&mut board, &[A2, A3, B1, B2, C2, C3]);

        let expected = "   | X | O \n---+---+---\n X | O |   \n---+---+---\n   | X | O ";
        assert_eq!(render(&board), expected);
    }

    #[test]
    fn to_string_appends_to_existing_content() {
        let mut output = String::from("Board:\n");
        Board::new().to_string(&mut output);

        let expected =
            "Board:\n   |   |   \n---+---+---\n   |   |   \n---+---+---\n   |   |   ";
        assert_eq!(output, expected);
    }
}
