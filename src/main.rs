pub mod gameBoard;
pub mod menu;

use gameBoard::GameBoard;

use std::io::stdin;

fn main() {
  let mut game = GameBoard::new();
  // draw_ascii();
   game.fill_in();
    game.display();
  let mut input = String::new();
  stdin().read_line(&mut input).expect("please input an valid key"); 
    
   menu::start_game(game, input.pop().unwrap())
}

//opening
fn draw_ascii() {
    let s_2048 = r#" /\\\\\\\\\          /\\\\\\\                /\\\         /\\\\\\\\\
  /\\\///////\\\      /\\\/////\\\            /\\\\\       /\\\///////\\\
  \///      \//\\\    /\\\    \//\\\         /\\\/\\\      \/\\\     \/\\\
             /\\\/    \/\\\     \/\\\       /\\\/\/\\\      \///\\\\\\\\\/
           /\\\//      \/\\\     \/\\\     /\\\/  \/\\\       /\\\///////\\\
         /\\\//         \/\\\     \/\\\   /\\\\\\\\\\\\\\\\   /\\\      \//\\\
        /\\\/            \//\\\    /\\\   \///////////\\\//   \//\\\      /\\\
        /\\\\\\\\\\\\\\\   \///\\\\\\\/              \/\\\      \///\\\\\\\\\/
        \///////////////      \///////                \///         \///////// 
        \n"#;

    println!("{}", s_2048);
}


fn draw_board()  {
    let mut board = GameBoard::new();
    board.fill_in();
    board.display()
}


//键盘控制
// 1.得读取键盘字母，wasd是上下左右，r是重启，c是保存


