use std::io::{BufRead, BufReader};

fn main() {
  play();
}

fn get_input(player: &Player, reader: &mut dyn BufRead) -> Result<(usize, usize), String> {
  fn get_input_for(prompt: &str, player: &Player, reader: &mut dyn BufRead) -> Result<usize, String> {
    println!("Please enter {} for player {}: ", prompt, player_to_string(*player));
    let mut input = String::new();
    reader.read_line(&mut input).map_err(|e| e.to_string())?;
    match input.trim().parse::<usize>() {
        Ok(num) if num <= 2 => {
            return Ok(num);
        }
        _ => return Err(format!("Invalid input for {}!", prompt)),
    }
  }

  let row = get_input_for("row", player, reader);
  if row.is_err() {
      return Err(row.err().unwrap());
  }

  let col = get_input_for("col", player, reader);
  if col.is_err() {
    return Err(col.err().unwrap());
  }

  Ok((row.unwrap(), col.unwrap()))
}

fn player_to_string(player: Player) -> String {
  match player {
    Player::X => "X".to_string(),
    Player::O => "O".to_string(),
  }
}

fn print_board(board: &Board) {
  for row in 0..3 {
    for col in 0..3 {
      let cell = board.get_cell(row, col);
      let cell_str = match cell {
        Some(player) => player_to_string(player),
        None => " ".to_string(),
      };
      print!("{}", cell_str);
      if col < 2 {
        print!("|");
      }
    }
    println!();
    if row < 2 {
      println!("-----");
    }
  }
}

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
enum Player {
  X,
  O,
}

fn play() {
  // initialize game state
let mut board = Board::new();
let mut current_player = Player::X;

  // main game loop
  loop {
      // print board
      print_board(&board);

      // get input from player from stdin
      let mut reader = BufReader::new(std::io::stdin());
      let (row, col) = match get_input(&current_player, &mut reader) {
          Ok((row, col)) => (row, col),
          Err(err) => {
              println!("Error: {}", err);
              continue;
          }
      };

      // update board
      let result = board.set_cell(row, col, current_player);

      if result.is_err() {
          println!("Error: {}", result.err().unwrap());
          continue;
      }

      // check if player has won
      if board.has_won(current_player) {
          println!("{} has won!", player_to_string(current_player));
          break;
      }

      // check if board is full
      if board.is_full() {
          println!("The game is a tie!");
          break;
      }

      // switch player
      current_player = match current_player {
          Player::X => Player::O,
          Player::O => Player::X,
      };
  }
}

struct Board {
    cells: [[Option<Player>; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board { cells: [[None; 3]; 3] }
    }

    fn get_cell(&self, row: usize, col: usize) -> Option<Player> {
        self.cells[row][col]
    }

    fn set_cell(&mut self, row: usize, col: usize, player: Player) -> Result<(), String> {
        if self.cells[row][col].is_some() {
            return Err("Cell is already occupied".to_string());
        }
        self.cells[row][col] = Some(player);
        Ok(())
    }

    fn is_full(&self) -> bool {
        for row in self.cells.iter() {
            for cell in row.iter() {
                if cell.is_none() {
                    return false;
                }
            }
        }
        true
    }

    fn has_won(&self, player: Player) -> bool {
        // check rows
        for row in self.cells {
            if row.iter().all(|&x| x == Some(player)) {
                return true;
            }
        }

        // check columns
        for i in 0..3 {
            if self.cells[0][i] == Some(player) && self.cells[1][i] == Some(player) && self.cells[2][i] == Some(player) {
                return true;
            }
        }

        // check diagonals
        if self.cells[0][0] == Some(player) && self.cells[1][1] == Some(player) && self.cells[2][2] == Some(player) {
            return true;
        }

        if self.cells[0][2] == Some(player) && self.cells[1][1] == Some(player) && self.cells[2][0] == Some(player) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_creates_an_empty_board() {
    let board = Board::new();
    // check that all cells are empty
    for row in board.cells.iter() {
      for cell in row.iter() {
        assert_eq!(None, *cell);
      }
    }
  }

  #[test]
  fn it_returns_false_if_no_one_has_won() {
    let board = Board::new();
    assert_eq!(false, board.has_won(Player::X));
    assert_eq!(false, board.has_won(Player::O));
  }

  #[test]
  fn it_returns_true_for_x_if_x_has_won() {
    let mut board = Board::new();
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(0, 1, Player::X));
    assert_eq!(Ok(()), board.set_cell(0, 2, Player::X));
    assert_eq!(true, board.has_won(Player::X));
  }

  #[test]
  fn it_returns_false_for_o_if_x_has_won() {
    let mut board = Board::new();
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(0, 1, Player::X));
    assert_eq!(Ok(()), board.set_cell(0, 2, Player::X));
    assert_eq!(false, board.has_won(Player::O));
  }

  #[test]
  fn it_returns_true_for_x_if_x_has_won_in_a_column() {
    let mut board = Board::new();
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(1, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(2, 0, Player::X));
    assert_eq!(true, board.has_won(Player::X));
  }

  #[test]
  fn it_returns_false_for_o_if_x_has_won_in_a_column() {
    let mut board = Board::new();
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(1, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(2, 0, Player::X));
    assert_eq!(false, board.has_won(Player::O));
  }

  #[test]
  fn it_returns_true_for_x_if_x_has_won_in_a_diagonal() {
    let mut board = Board::new();
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(1, 1, Player::X));
    assert_eq!(Ok(()), board.set_cell(2, 2, Player::X));
    assert_eq!(true, board.has_won(Player::X));
  } 

  #[test]
  fn it_returns_false_for_o_if_x_has_won_in_a_diagonal() {
    let mut board = Board::new();
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(1, 1, Player::X));
    assert_eq!(Ok(()), board.set_cell(2, 2, Player::X));
    assert_eq!(false, board.has_won(Player::O));
  }

  #[test]
  fn it_returns_the_value_of_the_cell_at_the_given_row_and_column() {
    let mut board = Board::new();
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(Some(Player::X), board.get_cell(0, 0));
    assert_eq!(Ok(()), board.set_cell(1, 2, Player::O));
    assert_eq!(Some(Player::O), board.get_cell(1, 2));
  }

  #[test]
  fn it_returns_true_if_the_board_is_full() {
    // write test code that checks if the board is full
    let mut board = Board::new();
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(0, 1, Player::X));
    assert_eq!(Ok(()), board.set_cell(0, 2, Player::X));
    assert_eq!(Ok(()), board.set_cell(1, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(1, 1, Player::X));
    assert_eq!(Ok(()), board.set_cell(1, 2, Player::X));
    assert_eq!(Ok(()), board.set_cell(2, 0, Player::X));
    assert_eq!(Ok(()), board.set_cell(2, 1, Player::X));
    assert_eq!(Ok(()), board.set_cell(2, 2, Player::X));
    assert_eq!(true, board.is_full());
  }

  #[test]
  fn it_returns_false_if_the_board_is_full() {
    // write test code that checks if the board is full
    let mut board = Board::new();
    assert_eq!(false, board.is_full());
    assert_eq!(Ok(()), board.set_cell(0, 0, Player::X));
    assert_eq!(false, board.is_full());
    assert_eq!(Ok(()), board.set_cell(0, 1, Player::X));
    assert_eq!(false, board.is_full());
    assert_eq!(Ok(()), board.set_cell(0, 2, Player::X));
    assert_eq!(Ok(()), board.set_cell(2, 2, Player::O));
    assert_eq!(false, board.is_full());
  }

  #[test]
  fn it_returns_valid_input_given_valid_input() {
    let mut reader = BufReader::new("1\n2\n".as_bytes());
    let (row, col) = get_input(&Player::X, &mut reader).unwrap();
    assert_eq!(1, row);
    assert_eq!(2, col);
  }

  #[test]
  fn it_returns_an_error_given_an_invalid_row() {
    let mut reader = BufReader::new("3\n1\n".as_bytes());
    let result = get_input(&Player::X, &mut reader);
    assert!(result.is_err());
    assert_eq!("Invalid input for row!", result.err().unwrap());
  }
}