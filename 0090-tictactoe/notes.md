## Module `tictactoe`

* Enum `Player`
  * `X`
  * `O`
* Enum `GameStatus`
  * `Winner(Player)`
  * `Draw`
  * `OngoingGame`
* Struct `BoardIndex` - holds a VALID index in the board
  * `col: u8`
  * `row: u8`
  * `fn new`
    * Input: col, row
    * Output: `Option<BoardIndex>`
* Enum `PlaceStoneError`
  * `AlreadyOccupied`
  * `GameAlreadyOver` (won or drawn)
* Struct `Board` - contains 9 instances of `Option<Player>`
  * `[[Option<Player>; 3]; 3]` - 3x3 array for fields
  * `next_player: Player` - player who has to place next stone
  * `game_state: GameStatus`
  * `fn place_stone` - places a stone and indicates the game status back
    * Input: `&mut self`, `pos: BoardIndex`
    * Output: `Result<GameStatus, PlaceStoneError>`
  * `fn to_string` - turns the board into a string that could be printed in logs, console, etc.
    * Input: `&self`, `&mut String`

```
   | X | O
---+---+---
 X | O |
---+---+---
   | X | O  
```

## Module `main`

* `fn main`
  * Game loop
    * Ask the user for her next move
      * If it is a number -> num-pad-style (e.g. 1=left lower, 5=middle, 9=right upper)
      * If it has 2 chars and the first is A-C and the second is 1-3 -> Chess-style (A-C for rows, 1-3 for cols)
    * Call `Board.place_stone`
    * if error -> print error
    * Show the board on the console (`to_string`)
    * if winner or draw -> print game result and end game
* Enum `ParseBoardIndexError`
  * `FormatNotSupported` - invalid string format (e.g. too long, does not match any of the defined formats)
  * `OutOfRange`
* `fn parse_board_index`
  * Input: `&str`
  * Output: `Result<BoardIndex, ParseBoardIndexError>`
