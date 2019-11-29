use crate::board;
use crate::solvers;
use crate::solvers::brute_force_solver;

#[test]
fn brute_force_solver1() {
    let board = board::Board {
        red_remain: 2,
        blue_remain: 5,
        data: [
            [0,  1,  0,  0,  0],
            [0, -5,  0,  0,  0],
            [0,  0,  0,  0, -6],
            [0,  0,  0,  5, -4],
            [0,  0, -3, -2,  0]
        ]
    };
    let ans = brute_force_solver::solve(&board, 5);
    assert_eq!(ans, solvers::SolveInfo {
        best_move: 335,
        comment: String::from("1")
    });
}

#[test]
fn brute_force_solver2() {
    let board = board::Board {
        red_remain: 2,
        blue_remain: 1,
        data: [
            [0,  1,  0,  0,  0],
            [0, -5,  0,  0,  0],
            [0,  0,  0,  0,  0],
            [0,  0,  0,  5,  0],
            [0,  0,  0,  0,  0]
        ]
    };
    let ans = brute_force_solver::solve(&board, 1);
    assert_eq!(ans, solvers::SolveInfo {
        best_move: 016,
        comment: String::from("1")
    });
}

#[test]
fn brute_force_solver3() {
    let board = board::Board {
        red_remain: 2,
        blue_remain: 1,
        data: [
            [0,  1,  0,  0,  0],
            [0, -5,  0,  0,  0],
            [0,  0,  0,  0,  0],
            [0,  0,  0,  5,  0],
            [0,  0,  0,  0,  0]
        ]
    };
    let ans = brute_force_solver::solve(&board, 1);
    assert_eq!(ans, solvers::SolveInfo {
        best_move: 016,
        comment: String::from("1")
    });
}

#[test]
fn brute_force_solver4() {
    let board = board::Board {
        red_remain: 2,
        blue_remain: 1,
        data: [
            [0,  0,  0,  0,  0],
            [0,  0, -5,  0,  0],
            [0,  0,  1,  0,  0],
            [0,  0,  0,  5,  0],
            [0,  0,  0,  0,  0]
        ]
    };
    let ans = brute_force_solver::solve(&board, 1);
    assert_eq!(ans.best_move, 225);
}

#[test]
fn brute_force_solver5() {
    let board = board::Board {
        red_remain: 2,
        blue_remain: 2,
        data: [
            [0,  0,  0,  0,  0],
            [0, -4, -5,  0,  0],
            [0,  0,  1,  0,  0],
            [0,  0,  0,  5,  0],
            [0,  0,  0,  0,  0]
        ]
    };
    let ans = brute_force_solver::solve(&board, 1);
    assert_eq!(ans.best_move, 225);
}