#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum Player {
  X,
  O,
}

pub fn player_to_string(player: Player) -> String {
  match player {
    Player::X => "X".to_string(),
    Player::O => "O".to_string(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_returns_x_given_x() {
    assert_eq!("X", player_to_string(Player::X));
  }

  #[test]
  fn it_returns_o_given_o() {
    assert_eq!("O", player_to_string(Player::O));
  }
}

