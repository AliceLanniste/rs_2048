
use rand::prelude::*;

#[derive(Debug)]
pub struct GameBoard {
   
   board: [[Option<i32>;4];4],
}

impl  GameBoard {
    fn new() -> Self {
       
       Self {
         board: [[None;4];4],
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


   pub fn display()  {
        let line =  "  +------+------+------+------+";
        let delimiter = "  | ";
        for _i in 0..=3 {
            println!("{}",line);
            
            for _j in 0..=3 {
                print!("{:width$}",delimiter,width=7);
            }

            println!("{}",delimiter);
        }

        println!("{}",line);
    }

  //随机选位置[x][y],然后随机选2或4，2出现的概率要大于4
    fn fill_in(&mut self)  {
        let area = self.position();
        let mut rng = thread_rng();

        match area.choose(&mut rng) {
            Some(tuple) => {
                let (x,y) = *tuple;
                self.board[x][y] = Some(GameBoard::rand_number());
            },
            None => {},
        }
       
    }
    
    fn position(&self) -> Vec<(usize,usize)> {
        let mut empty_position = Vec::new();
       for i in 0..=3 {
           for j in 0..=3 {
               if self.board[i][j] ==None {
                   empty_position.push((i,j));
               }
           }
       }
        
        empty_position
    }

    //产生2和4
    fn rand_number() -> i32 {
        let  choices: [i32; 4] = [2, 2, 2, 4];
        let mut rng = thread_rng();

        *choices.choose(&mut rng).unwrap()
        
    }

    fn testAdd(&self,x:usize,y:usize, value: i32) ->bool{
        if x >3 || y > 3 {
            return false;
        }

        Some(value) == self.board[x][y] 
       
    }

  
}
