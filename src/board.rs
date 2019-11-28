use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    pub red_remain: i32,
    pub blue_remain: i32,
    pub data: [[i32; 5]; 5],
}

const DIR: [[i32; 2]; 8] = [
    [0, -1],
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
];


// constructors
impl Board {
    pub fn new() -> Board {
        Board {
            red_remain: 6,
            blue_remain: 6,
            data: [
                [1,  2,  3,  0,  0],
                [4,  5,  0,  0,  0],
                [6,  0,  0,  0, -6],
                [0,  0,  0, -5, -4],
                [0,  0, -3, -2, -1]
            ],
        }
    }
}


// methods
impl Board {
    pub fn mov(&mut self, x: i32, y: i32, dir: i32) {
        let nx = (x + DIR[dir as usize][0]) as u32;
        let ny = (y + DIR[dir as usize][1]) as u32;

        let last = self.data[x as usize][y as usize];
        self.data[x as usize][y as usize] = 0;
        if self.data[nx as usize][ny as usize] > 0 {
            self.red_remain -= 1;
        } else if self.data[nx as usize][ny as usize] < 0 {
            self.blue_remain -= 1;
        }

        self.data[nx as usize][ny as usize] = last;
    }

    pub fn get(&self, x: i32, y: i32) -> i32 {
        self.data[x as usize][y as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, val: i32) {
        self.data[x as usize][y as usize] = val;
    }

    pub fn get_blue_remain(&self) -> i32 {
        self.blue_remain
    }

    pub fn get_red_remain(&self) -> i32 {
        self.red_remain
    }
}


// traits
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", 666)
    }
}



pub fn winner(board: &Board) -> i32 {
    if board.get(0, 0) < 0 || board.get_red_remain() == 0 {
        2
    } else if board.get(4, 4) > 0 || board.get_blue_remain() == 0 {
        1
    } else {
        0
    }
}

pub fn valid_move(board: &Board, dice: i32, player: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let mut temp = [0_i32; 7];
    let mut temp_pos = [[0_i32; 2]; 7];
    for i in 0..5 {
        for j in 0..5 {
            let val = board.get(i as i32, j as i32);
            if val > 0 && player == 1 {
                temp[val as usize] = 1;
                temp_pos[val as usize][0] = i;
                temp_pos[val as usize][1] = j;
            } else if val < 0 && player == 2 {
                temp[-val as usize] = 1;
                temp_pos[-val as usize][0] = i;
                temp_pos[-val as usize][1] = j;
            }
        }
    }

    if temp[dice as usize] == 1 {
        let l: i32 = if player == 1 { 4 } else { 0 };
        let r: i32 = if player == 1 { 6 } else { 2 };
        for i in l..=r {
            let nx = temp_pos[dice as usize][0] + DIR[i as usize][0];
            let ny = temp_pos[dice as usize][1] + DIR[i as usize][1];
            if nx >= 0 && nx < 5 && ny >= 0 && ny < 5 {
                ans.push(temp_pos[dice as usize][0] * 100 + temp_pos[dice as usize][1] * 10 + i);
            }
        }
        return ans;
    }

    let mut it = dice - 1;
    while it >= 1 && temp[it as usize] == 0 {
        it -= 1;
    }
    if it >= 1 {
        let l: i32 = if player == 1 { 4 } else { 0 };
        let r: i32 = if player == 1 { 6 } else { 2 };
        for i in l..=r {
            let nx = temp_pos[it as usize][0] + DIR[i as usize][0];
            let ny = temp_pos[it as usize][1] + DIR[i as usize][1];
            if nx >= 0 && nx < 5 && ny >= 0 && ny < 5 {
                ans.push(temp_pos[it as usize][0] * 100 + temp_pos[it as usize][1] * 10 + i);
            }
        }
    }

    it = dice + 1;
    while it <= 6 && temp[it as usize] == 0 {
        it += 1;
    }
    if it <= 6 {
        let l: i32 = if player == 1 { 4 } else { 0 };
        let r: i32 = if player == 1 { 6 } else { 2 };
        for i in l..=r {
            let nx = temp_pos[it as usize][0] + DIR[i as usize][0];
            let ny = temp_pos[it as usize][1] + DIR[i as usize][1];
            if nx >= 0 && nx < 5 && ny >= 0 && ny < 5 {
                ans.push(temp_pos[it as usize][0] * 100 + temp_pos[it as usize][1] * 10 + i);
            }
        }
    }
    
    ans
}

pub fn set_layout(board: &mut Board, player: i32, layout: &str) {
    let layout: Vec<i32> = layout.chars().map(|x| x as i32).collect();
    if player == 1 {
        board.set(0, 0, layout[0]);
        board.set(0, 1, layout[1]);
        board.set(0, 2, layout[2]);
        board.set(1, 0, layout[3]);
        board.set(1, 1, layout[4]);
        board.set(2, 0, layout[5]);
    } else if player == 2 {
        board.set(4, 4, -layout[0]);
        board.set(4, 3, -layout[1]);
        board.set(4, 2, -layout[2]);
        board.set(3, 4, -layout[3]);
        board.set(3, 3, -layout[4]);
        board.set(2, 4, -layout[5]);
    }
}

pub fn flip_coord(x: &mut i32, y: &mut i32) {
    *x = 4 - *x;
    *y = 4 - *y;
}