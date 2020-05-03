use rand::prelude::{thread_rng, SliceRandom};

#[derive(Debug)]
pub struct GameBoard {
    pub score: u32,
    pub largest:u32,
    board: [[Option<i32>; 4]; 4],
}

#[derive(Debug)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            score: 0,
            largest:0,
            board: [[None; 4]; 4],
        }
    }
    //    +------+------+------+------+
    //    | (0,0)|      |      | (0,3)|
    //    +------+------+------+------+
    //    | (1,0)|      |      | (1,3)|
    //    +------+------+------+------+
    //    | (2,0)|      |      | (2,3)|
    //    +------+------+------+------+
    //    | (3,0)|      |      | (3,3)|
    //    +------+------+------+------+

    pub fn display(&self) {
        let line = "  +------+------+------+------+";
        let delimiter = "  | ";
        for i in 0..=3 {
            println!("{}", line);

            for j in 0..=3 {
                match self.board[i][j] {
                    Some(value) => print!("  |{:width$}", value,width = 4),
                    None => print!("{:width$}", delimiter, width = 7),
                }
            }

            println!("{}", delimiter);
        }

        println!("{}", line);
    }

    //随机选位置[x][y],然后随机选2或4，2出现的概率要大于4
    pub fn fill_in(&mut self) {
        let area = self.position();
        let mut rng = thread_rng();

        match area.choose(&mut rng) {
            Some(tuple) => {
                let (x, y) = *tuple;
                self.board[x][y] = Some(GameBoard::rand_number());
            }
            None => {}
        }
    }

    fn position(&self) -> Vec<(usize, usize)> {
        let mut empty_position = Vec::new();
        for i in 0..=3 {
            for j in 0..=3 {
                if self.board[i][j] == None {
                    empty_position.push((i, j));
                }
            }
        }

        empty_position
    }

    //产生2和4
    fn rand_number() -> i32 {
        let choices: [i32; 4] = [2, 2, 2, 4];
        let mut rng = thread_rng();

        *choices.choose(&mut rng).unwrap()
    }

    

    pub fn direction_move(&mut self, direct: Directions) {
        match direct {
            Directions::UP => {
                for x in 0..=3 {
                    let mut y = 1;
                    while y < 4 {
                        self.rs_move(x, y, 0, -1);
                        y += 1;
                    }
                }
            }
            Directions::DOWN => {
                for x in 0..=3 {
                    let mut y = 2;
                    while y >= 0 {
                        self.rs_move(x, y, 0, 1);
                        y -= 1;
                    }
                }
            }
            Directions::LEFT => {
                for y in 0..=3 {
                    let mut x = 1;
                    while x < 4 {
                        self.rs_move(x, y, -1, 0);
                        x += 1;
                    }
                }
            }
            Directions::RIGHT => {
                for y in 0..=3 {
                    let mut x = 2;
                       while x >= 0 {
                          self.rs_move(x, y, 1, 0);
                          x -= 1;
                       }
                           
                      
                }
            }
        }
    }

    fn rs_move(&mut self, x: i8, y: i8, c: i8, r: i8) {
        let (x1, y1, tx, ty) = (x as usize, y as usize, (x + c) as usize, (y + r) as usize);

        
         let current = self.board[y1][x1];
         let target = self.board[ty][tx];
    
        
        
        match (current, target) {
            (None, _) => {}
            (Some(v1), Some(v2)) if v1 == v2 => {
                self.board[ty][tx] = self.board[ty][tx].map(|v| v * 2);
                self.board[y1][x1] = None;
                let target = if let Some(number) = self.board[ty][tx] {
                    number as u32
                }else { unreachable!() };
                
                self.score += target;
                self.largest = if self.largest < target  {target} else {self.largest}; 
                if target == 2048 {
                    println!("you Win! ");
                    
                }

            }
            (Some(v1), None) => {
                  self.board[ty][tx] = Some(v1);
                  self.board[y1][x1] = None;
                 if r + c == 1 && if r == 1 { y } else { x } <2 {
                      self.rs_move(x + c, y + r, c, r) 
                } else if r + c == -1 && if r == -1 { y } else { x } > 1 {
                    self.rs_move(x + c, y + r, c, r);
                }
            }
            (Some(_), Some(_)) => {}
        }
    }


   

}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_move_up() {
        let mut test_board = GameBoard::new();
        let test_data = [
            [Some(2), None, None,None],
            [None, None, None, None],
            [None, None, None, None],
            [None, None, None,    None],
        ];
        test_board.board = [
            [None, None, None, None],
            [None, None, None, None],
            [None, None, None, None],
            [Some(2), None, None,    None],
        ];
        test_board.direction_move(Directions::UP);
        assert_eq!(test_board.board, test_data);
    }



    #[test]
    fn test_move_down() {
        let mut test_board = GameBoard::new();
        let test_data = [
            [None, None, None, None],
            [None, None, None, None],
            [None, None, None, None],
            [Some(4), None, None,Some(2)],
        ];
        test_board.board = [
            [Some(2), None, None, Some(2)],
            [None,    None, None, None],
            [None,    None, None, None],
            [Some(2), None, None,    None],
        ];
        test_board.direction_move(Directions::DOWN);
        assert_eq!(test_board.board, test_data);
    }

    #[test]
    fn test_move_right() {
        let mut test_board = GameBoard::new();
        let test_data = [
            [None, None, None, Some(2)],
            [None, None, None, None],
            [None, None, None, None],
            [None, None, None, None],
        ];
        test_board.board = [
            [Some(2), None,None,None],
            [None, None,None,None],
            [None, None,None,None],
           [None, None,None,None]
        ];
        test_board.direction_move(Directions::RIGHT);
        assert_eq!(test_board.board, test_data);
    }



    #[test]
    fn test_move_left() {
        let mut test_board = GameBoard::new();
        let test_data = [
            [Some(4), None, None, None],
            [None,    None, None, None],
            [None,    None, None, None],
            [Some(2), None, None,  None ],
        ];
        test_board.board = [
            [Some(2), None, None, Some(2)],
            [None,    None, None, None],
            [None,    None, None, None],
            [Some(2), None, None,    None],
        ];
        test_board.direction_move(Directions::LEFT);
        assert_eq!(test_board.board, test_data);
    }
}


