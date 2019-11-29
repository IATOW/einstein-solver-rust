use crate::board;
use rand::prelude::*;

pub fn simulate(board: &board::Board, player: i32, num: i32) -> i32 {
    let mut ans = 0_i32;
    let mut rng = rand::thread_rng();

    for _ in 0..num {
        let mut b = board.clone();
        let mut turn = player;
        let mut w = 0_i32;
        
        loop {
            let dice = rng.gen_range(0, 6);
            let valid_move = board::valid_move(&b, dice, turn);

            let idx = rng.gen_range(0, valid_move.len());
            let action = valid_move[idx as usize];

            let x = action / 100;
            let y = action / 10 % 10;
            let dir = action % 10;

            b.mov(x, y, dir);
            w = board::winner(&b);
            if w != 0 {
                break;
            }

            turn = 3 - turn;
        }

        if w == 1 {
            ans += 1;
        }
    }

    ans
}