use crate::gameBoard::{GameBoard,Directions};

use std::io::stdin;

pub fn start_game()  {
    let mut game = GameBoard::new();
     game.fill_in();
      game.display();
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
          Ok(_) => {
            if input.starts_with('w') || input.starts_with('W') {
                game.Direction_move(Directions::UP);
                game.fill_in();
                game.display();
            } else if input.starts_with('s') || input.starts_with('S') {
              game.Direction_move(Directions::DOWN);
                game.fill_in();
                game.display();
            } else if input.starts_with('a') || input.starts_with('A') {
              game.Direction_move(Directions::LEFT);
              game.fill_in();
              game.display();
            }  else if input.starts_with('d') || input.starts_with('D') {
              game.Direction_move(Directions::RIGHT);
              game.fill_in();
              game.display();
            } else if input.starts_with('x') || input.starts_with('X') {
              break;
            }else {
              println!("invalid input");
              break;
            }
          },
          Err(e) => { println!("error, {}",e); break },
        }
      }  
}

// fn continue_game(arg: Type) -> RetType {
//     unimplemented!();
// }


// fn exit_game(arg: Type) -> RetType {
//     unimplemented!();
// }

