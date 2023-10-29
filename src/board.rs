
use crate::Player;

pub struct Board {
  cells: [[Option<Player>; 3]; 3],
}

impl Board {
  pub fn new() -> Board {
      Board { cells: [[None; 3]; 3] }
  }

  pub fn get_cell(&self, row: usize, col: usize) -> Option<Player> {
      self.cells[row][col]
  }

  pub fn set_cell(&mut self, row: usize, col: usize, player: Player) -> Result<(), String> {
      if self.cells[row][col].is_some() {
          return Err("Cell is already occupied".to_string());
      }
      self.cells[row][col] = Some(player);
      Ok(())
  }

  pub fn is_full(&self) -> bool {
      for row in self.cells.iter() {
          for cell in row.iter() {
              if cell.is_none() {
                  return false;
              }
          }
      }
      true
  }

  pub fn has_won(&self, player: Player) -> bool {
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
}