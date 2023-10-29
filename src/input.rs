use crate::Player;
use crate::player_to_string;
use std::io::BufRead;


pub fn get_input(player: &Player, reader: &mut dyn BufRead) -> Result<(usize, usize), String> {
  fn get_input_for(prompt: &str, reader: &mut dyn BufRead) -> Result<usize, String> {
    let mut input = String::new();
    reader.read_line(&mut input).map_err(|e| e.to_string())?;
    match input.trim().parse::<usize>() {
        Ok(num) if num <= 2 => {
            return Ok(num);
        }
        _ => return Err(format!("Invalid input for {}!", prompt)),
    }
  }

  println!("Please enter row for player {}: ", player_to_string(*player));
  let row = get_input_for("row", reader);
  if row.is_err() {
      return Err(row.err().unwrap());
  }

  println!("Please enter col for player {}: ", player_to_string(*player));
  let col = get_input_for("col", reader);
  if col.is_err() {
    return Err(col.err().unwrap());
  }

  Ok((row.unwrap(), col.unwrap()))
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::io::BufReader;

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