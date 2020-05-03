use crate::gameBoard::{GameBoard,Directions};

use std::io::stdin;

pub fn start_game()  {
    let mut game = GameBoard::new();
     game.fill_in();
      game.display();
    loop {
        let mut input = String::new();
        let _ =stdin().read_line(&mut input);
          match input.trim() {
              "w" |"W"=> game.direction_move(Directions::UP),
              "s" |"S"=> game.direction_move(Directions::DOWN),
              "a" |"A"=> game.direction_move(Directions::LEFT),
              "d" |"D"=> game.direction_move(Directions::RIGHT),

              _ => break,
          }
     
        game.fill_in();
        game.display();
      }  
}


