fn main() {
  println!("{}", hello_world());
}

fn hello_world() -> String {
  return "Hello World!".to_string();
}

fn validate_input(input: &str) -> bool {
  return input == "X" || input == "O";
}

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
enum Player {
  X,
  O,
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
  fn it_returns_hello_world() {
      assert_eq!("Hello World!", hello_world());
  }

  #[test]
  fn it_returns_true_if_input_is_x_or_o() {
    assert_eq!(true, validate_input("X"));
    assert_eq!(true, validate_input("O"));
  }

  #[test] 
  fn it_returns_false_if_input_is_not_x_or_o() {
    assert_eq!(false, validate_input("A"));
    assert_eq!(false, validate_input("B"));
  }

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
}