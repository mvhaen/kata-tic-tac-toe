fn main() {
  println!("{}", hello_world());
}

fn hello_world() -> String {
  return "Hello World!".to_string();
}

type Board = [[Option<Player>; 3]; 3];

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
enum Player {
  X,
  O,
}

fn create_board() -> Board {
  return [[None; 3]; 3];
}

fn validate_input(input: &str) -> bool {
  return input == "X" || input == "O";
}

fn has_won(board: Board, player: Player) -> bool {
  // check rows
  for row in board {
    if row.iter().all(|&x| x == Some(player)) {
      return true;
    }
  }

  // check columns
  for i in 0..3 {
    if board[0][i] == Some(player) && board[1][i] == Some(player) && board[2][i] == Some(player) {
      return true;
    }
  }

  // check diagonals
  if board[0][0] == Some(player) && board[1][1] == Some(player) && board[2][2] == Some(player) {
    return true;
  }

  if board[0][2] == Some(player) && board[1][1] == Some(player) && board[2][0] == Some(player) {
    return true;
  }
  return false;
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
  fn it_creates_a_board() {
    let board = create_board();
    assert_eq!(board, [[None, None, None], [None, None, None], [None, None, None]]);
  }

  #[test]
  fn it_returns_false_if_no_one_has_won() {
    let board = create_board();
    assert_eq!(false, has_won(board, Player::X));
    assert_eq!(false, has_won(board, Player::O));
  }

  #[test]
  fn it_returns_true_for_x_if_x_has_won() {
    let mut board = create_board();
    board[0][0] = Some(Player::X);
    board[0][1] = Some(Player::X);
    board[0][2] = Some(Player::X);
    assert_eq!(true, has_won(board, Player::X));
  }

  #[test]
  fn it_returns_false_for_o_if_x_has_won() {
    let mut board = create_board();
    board[0][0] = Some(Player::X);
    board[0][1] = Some(Player::X);
    board[0][2] = Some(Player::X);
    assert_eq!(false, has_won(board, Player::O));
  }

  // generate a test for a column win for X
  // generate a test for a column win for O
  // generate a test for a diagonal win for X
  // generate a test for a diagonal win for O
  #[test]
  fn it_returns_true_for_x_if_x_has_won_in_a_column() {
    let mut board = create_board();
    board[0][0] = Some(Player::X);
    board[1][0] = Some(Player::X);
    board[2][0] = Some(Player::X);
    assert_eq!(true, has_won(board, Player::X));
  }

  #[test]
  fn it_returns_false_for_o_if_x_has_won_in_a_column() {
    let mut board = create_board();
    board[0][0] = Some(Player::X);
    board[1][0] = Some(Player::X);
    board[2][0] = Some(Player::X);
    assert_eq!(false, has_won(board, Player::O));
  }

  #[test]
  fn it_returns_true_for_x_if_x_has_won_in_a_diagonal() {
    let mut board = create_board();
    board[0][0] = Some(Player::X);
    board[1][1] = Some(Player::X);
    board[2][2] = Some(Player::X);
    assert_eq!(true, has_won(board, Player::X));
  } 

  #[test]
  fn it_returns_false_for_o_if_x_has_won_in_a_diagonal() {
    let mut board = create_board();
    board[0][0] = Some(Player::X);
    board[1][1] = Some(Player::X);
    board[2][2] = Some(Player::X);
    assert_eq!(false, has_won(board, Player::O));
  }
}