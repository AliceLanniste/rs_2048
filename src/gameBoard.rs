
use rand::prelude::{thread_rng, SliceRandom};

#[derive(Debug)]
pub struct GameBoard {
   
   board: [[Option<i32>;4];4],
}


#[derive(Debug)]
enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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


   pub fn display(&self)  {
        let line =  "  +------+------+------+------+";
        let delimiter = "  | ";
        for i in 0..=3 {
            println!("{}",line);
            
            for j in 0..=3 {
                match self.board[i][j] {
                    Some(value) =>  print!("{:width$}",value,width=7),
                    None =>  print!("{:width$}",delimiter,width=7),
                }
               
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

    fn test_add(&self,x:usize,y:usize, value: i32) ->bool{
        if x >3 || y > 3 {
            return false;
        }

        Some(value) == self.board[x][y] 
       
    }

    pub fn Direction_move(&mut self,dierct:Directions)  {
        match direct {
            UP => {for x in 0..=3 {
                let y = 1;
                while y < 4 {
                        self.move(x, y, 0, -1);
                    y++;
                }
            }},
            DOWN=> {for x in 0..=3 {
                let y = 2;
                while y >=0 {
                        self.move(x, y, 0, 1);
                    y--;
                }
            }},
            LEFT=> {for y in 0..=3 {
                let x = 1;
                while x < 4 {
                    self.move(x,y,-1,0);
                    x++;
                }
            }},
            RIGHT=>{ for y in 0..=3 {
                let x = 2;
                while x >= 0 {
                    self.move(x,y,1,0);
                    x--;
                }
            }},
        }

       
    }


    fn move(&mut self ,x:i8,y:i8,c:i8,r:i8)  {
        let (x1,y1 ,tx, ty) = (x as usize, y as usize, (x+c) as usize, (y+r) as usize);
        let current = self.board[y1][x1];
        let target = self.board[ty][tx];
        match (current, target) {
            (None, _)  => {},
            (Some(v1),Some(v2)) if v1 == v2 => {
                    self.board[ty][tx] = self.board[ty][tx].map(|v| v*2);
                    self.board[y1][x1]= None;},
            (Some(v1),None) => {
                if (r+c ==1 && if r==1 {y} else {x} <2 ){
                    move_down(x+c,y+r,c,r);}
                else if( r+c == -1 && if r == -1 {y} else {x} > 1){
                    move_down(x+c,y+r,c,r);
                }    
                }},
            (Some(_),Some(_))  => {},
        }

        
        unimplemented!();
    }
    

    pub  fn move_right(&self)  {
        unimplemented!();
    }

    pub fn is_up_movable(&self) -> bool {
        let mut movable = false;
        for x in 0..=3 {
            let row = [self.board[3][x],
                      self.board[2][x],
                      self.board[1][x],
                      self.board[0][x]];
            if GameBoard::is_movable(row) {
                movable = true;
                break
            }          
        }
        movable
    }



    pub fn is_down_movable(&self) -> bool {
        let mut movable = false;
        for x in 0..=3 {
            let row = [self.board[0][x],
                      self.board[1][x],
                      self.board[2][x],
                      self.board[3][x]];
            if GameBoard::is_movable(row) {
                movable = true;
                break
            }                      
        }
       
        movable
    }

    pub fn is_left_movable(&self) -> bool {
        let mut movable = false;
        for y in 0..=3 {
            let col = [self.board[y][3],
                       self.board[y][2],
                       self.board[y][1],
                       self.board[y][0]];
            if GameBoard::is_movable(col) {
                movable = true;
                break
            }
        }
        movable
    }

    pub fn is_right_movable(&self) -> bool {
        let mut movable = false;
        for y in 0..=3 {
            let col = [self.board[y][0],
                       self.board[y][1],
                       self.board[y][2],
                       self.board[y][3]];
            if GameBoard::is_movable(col) {
                movable = true;
                break
            }
        }
        movable
    }

    fn is_movable(row:[Option<i32>;4]) -> bool {
        match row {
           [None,None,None,_] => false,
           [None,Some(a),Some(b),Some(c)] if a!=b && b!=c  => false,
            [None,None,Some(b),Some(c)] if b !=c  =>false,
            [Some(a),Some(b),Some(c),Some(d)]  if a!=b &&b!=c && c!=d =>false,

            _  => true
        }
        
    }
}


//1.改变初始棋盘 
//2.可以左右移动以及如何判断左右移动
//3.游戏无法继续下去

