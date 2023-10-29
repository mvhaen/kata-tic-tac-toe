fn main() {
  println!("{}", hello_world());
}

fn hello_world() -> String {
  return "Hello World!".to_string();
}

type Board = [[Option<char>; 3]; 3];

fn create_board() -> Board {
  return [[None, None, None], [None, None, None], [None, None, None]];
}

fn validate_input(input: &str) -> bool {
  return input == "X" || input == "O";
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
}