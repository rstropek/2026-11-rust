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

pub struct BoardIndex {
    col: u8,
    row: u8,
}

impl BoardIndex {
    pub fn new(col: u8, row: u8) -> Option<Self> {
        if col < 3 && row < 3 {
            Some(Self { col, row })
        } else {
            None
        }
    }

    pub fn get_col(&self) -> u8 {
        self.col
    }

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
    use super::*;

    #[test]
    fn new_accepts_largest_valid_col() {
        let index = BoardIndex::new(2, 0).unwrap();
        assert_eq!(index.col, 2);
        assert_eq!(index.row, 0);
    }

    #[test]
    fn new_accepts_largest_valid_row() {
        let index = BoardIndex::new(0, 2).unwrap();
        assert_eq!(index.col, 0);
        assert_eq!(index.row, 2);
    }

    #[test]
    fn new_rejects_col_out_of_range() {
        assert!(BoardIndex::new(3, 0).is_none());
    }

    #[test]
    fn new_rejects_row_out_of_range() {
        assert!(BoardIndex::new(0, 3).is_none());
    }

    fn index(col: u8, row: u8) -> BoardIndex {
        BoardIndex::new(col, row).unwrap()
    }

    // Plays moves given as (col, row). Every move must leave the game ongoing.
    fn play(board: &mut Board, moves: &[(u8, u8)]) {
        for &(col, row) in moves {
            assert!(matches!(
                board.place_stone(index(col, row)),
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
        play(&mut board, &[(0, 0), (0, 1), (1, 0), (1, 1)]);
        assert!(matches!(
            board.place_stone(index(2, 0)),
            Ok(GameStatus::Winner(Player::X))
        ));
        board
    }

    fn drawn_board() -> Board {
        let mut board = Board::new();
        play(
            &mut board,
            &[
                (0, 0),
                (1, 0),
                (2, 0),
                (1, 1),
                (1, 2),
                (0, 2),
                (0, 1),
                (2, 1),
            ],
        );
        assert!(matches!(
            board.place_stone(index(2, 2)),
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
        play(&mut board, &[(0, 0), (1, 0), (2, 0)]);

        let expected = " X | O | X \n---+---+---\n   |   |   \n---+---+---\n   |   |   ";
        assert_eq!(render(&board), expected);
    }

    #[test]
    fn place_stone_rejects_occupied_field_and_keeps_turn() {
        let mut board = Board::new();
        play(&mut board, &[(0, 0)]);

        assert!(matches!(
            board.place_stone(index(0, 0)),
            Err(PlaceStoneError::AlreadyOccupied)
        ));

        play(&mut board, &[(1, 0)]);
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
        play(&mut board, &[(0, 0), (2, 0), (1, 0), (1, 1), (2, 2)]);

        assert!(matches!(
            board.place_stone(index(0, 2)),
            Ok(GameStatus::Winner(Player::O))
        ));
    }

    #[test]
    fn place_stone_prefers_winner_over_draw_on_last_field() {
        let mut board = Board::new();
        play(
            &mut board,
            &[
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (0, 2),
                (2, 1),
                (2, 2),
                (1, 2),
            ],
        );

        assert!(matches!(
            board.place_stone(index(1, 1)),
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
            board.place_stone(index(2, 2)),
            Err(PlaceStoneError::GameAlreadyOver)
        ));
    }

    #[test]
    fn place_stone_reports_game_over_before_occupied_field() {
        let mut board = board_won_by_x();
        assert!(matches!(
            board.place_stone(index(0, 0)),
            Err(PlaceStoneError::GameAlreadyOver)
        ));
    }

    #[test]
    fn place_stone_rejects_move_after_draw() {
        let mut board = drawn_board();
        assert!(matches!(
            board.place_stone(index(0, 0)),
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
        play(
            &mut board,
            &[(1, 0), (2, 0), (0, 1), (1, 1), (1, 2), (2, 2)],
        );

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
