use crate::board;
use crate::simulator;
use crate::constants;
use crate::solvers;


fn search(board: &board::Board, dep: i32, player: i32) -> f64 {
    let winner = board::winner(board);
    if winner == 1 {
        return if player == 1 { 1.0 } else { 0.0 };
    } else if winner == 2 {
        return if player == 1 { 0.0 } else { 1.0 };
    }

    if dep == 0 {
        let amount = 10;
        let win = simulator::simulate(board, player, amount);
        let rate = win as f64 / amount as f64;
        
        return if player == 1 { rate } else { 1.0 - rate };
    }

    let mut live_piece = [0_i32; 7];
    let mut live_piece_pos = [[0_i32; 2]; 7];
    for i in 0..5 {
        for j in 0..5 {
            let val = board.get(i as i32, j as i32);
            if val > 0 && player == 1 {
                live_piece[val as usize] = 1;
                live_piece_pos[val as usize][0] = i as i32;
                live_piece_pos[val as usize][1] = j as i32;
            } else if val < 0 && player == 2 {
                live_piece[-val as usize] = 1;
                live_piece_pos[-val as usize][0] = i as i32;
                live_piece_pos[-val as usize][1] = j as i32;
            }
        }
    }

    let mut rate = [0.0; 7];
    for i in 1..=6 {
        if live_piece[i] == 0 {
            continue;
        }

        let l = if player == 1 { 4 } else { 0 };
        let r = if player == 1 { 6 } else { 2 };

        for dir in l..=r {
            let nx = live_piece_pos[i][0] + constants::DIR[dir][0];
            let ny = live_piece_pos[i][1] + constants::DIR[dir][1];
            if nx >= 0 && nx < 5 && ny >= 0 && ny < 5 {
                let mut b = board.clone();
                b.mov(live_piece_pos[i][0], live_piece_pos[i][1], dir as i32);

                let val = 1.0 - search(&b, dep - 1, 3 - player);
                if val > rate[i] {
                    rate[i] = val;
                }
            }
        }
    }

    let mut ans = 0.0;
    for dice in 1..=6 {
        if live_piece[dice] == 1 {
            ans += rate[dice] / 6.0;
            continue;
        }

        let mut it = 0;
        let mut temp = 0.0;

        it = dice - 1;
        while it >= 1 && live_piece[it] == 0 {
            it -= 1;
        }
        if it >= 1 {
            temp = rate[it];
        }

        it = dice + 1;
        while it <= 6 && live_piece[it] == 0 {
            it += 1;
        }
        if it <= 6 {
            temp = temp.max(rate[it]);
        }

        ans += temp / 6.0;
    }

    ans
}

pub fn solve(board: &board::Board, dice: i32) -> solvers::SolveInfo {
    let mut live_piece = [0_i32; 7];
    let mut live_piece_pos = [[0_i32; 2]; 7];
    for i in 0..5 {
        for j in 0..5 {
            let val = board.get(i as i32, j as i32);
            if val > 0 {
                live_piece[val as usize] = 1;
                live_piece_pos[val as usize][0] = i as i32;
                live_piece_pos[val as usize][1] = j as i32;
            }
        }
    }

    let mut it = dice;
    while it >= 1 && live_piece[it as usize] == 0 {
        it -= 1;
    }
    if it >= 1 {
        live_piece[it as usize] = 2;
    }
    it = dice;
    while it <= 6 && live_piece[it as usize] == 0 {
        it += 1;
    }
    if it <= 6 {
        live_piece[it as usize] = 2;
    }

    let mut max_val = -constants::INF as f64;
    let mut best_move = 0_i32;
    for i in 1..=6 {
        if live_piece[i] != 2 {
            continue;
        }
        for dir in 4..=6 {
            let nx = live_piece_pos[i][0] + constants::DIR[dir][0];
            let ny = live_piece_pos[i][1] + constants::DIR[dir][1];
            if nx >= 0 && nx < 5 && ny >= 0 && ny < 5 {
                let mut b = board.clone();
                b.mov(live_piece_pos[i][0], live_piece_pos[i][1], dir as i32);
                let val = 1.0 - search(&b, 4, 2);
                if val > max_val {
                    max_val = val;
                    best_move = live_piece_pos[i][0] * 100 + live_piece_pos[i][1] * 10 + dir as i32;
                }
            }
        }
    }

    solvers::SolveInfo {
        best_move: best_move,
        comment: max_val.to_string(),
    }
}