mod player;
mod board;
mod input;
mod output;

use std::io::BufReader;
use player::{Player, player_to_string};
use board::Board;
use output::print_board;
use input::get_input;

fn main() {
  play();
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

