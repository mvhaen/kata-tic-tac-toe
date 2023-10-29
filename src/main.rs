mod player;
mod board;

use std::io::{BufRead, BufReader};
use player::{Player, player_to_string};
use board::Board;

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



#[cfg(test)]
mod tests {
  use super::*;

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