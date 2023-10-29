use crate::board::Board;
use crate::player_to_string;

pub fn print_board(board: &Board) {
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