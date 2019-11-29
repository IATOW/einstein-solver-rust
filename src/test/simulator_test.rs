use crate::simulator;
use crate::board;

#[test]
pub fn simulate1() {
    let board = board::Board {
        red_remain: 2,
        blue_remain: 5,
        data: [
            [1,  0,  0,  0,  0],
            [0,  -5,  0,  0,  0],
            [0,  0,  0,  0, -6],
            [0,  0,  0, 5, -4],
            [0,  0, -3, -2, 0]
        ]
    };
    
    let ans = simulator::simulate(&board, 1, 100);
    assert!(0 <= ans && ans <= 100);
    assert!(ans >= 50);
}