
use crate::gameBoard::GameBoard;


pub fn start_game(mut game:GameBoard,key:char)  {
    game.fill_in();
    game.display();
    loop {
       game.control(key)
    }
}

// fn continue_game(arg: Type) -> RetType {
//     unimplemented!();
// }


// fn exit_game(arg: Type) -> RetType {
//     unimplemented!();
// }

