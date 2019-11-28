use crate::board;

#[test]
fn new() {
    let board = board::Board::new();
    assert_eq!(board::Board {
        red_remain: 6,
        blue_remain: 6,
        data: [
            [1,  2,  3,  0,  0],
            [4,  5,  0,  0,  0],
            [6,  0,  0,  0, -6],
            [0,  0,  0, -5, -4],
            [0,  0, -3, -2, -1]
        ],
    }, board);
}

#[test]
fn mov() {
    let mut board = board::Board::new();
    board.mov(3, 3, 0);
    assert_eq!(board::Board {
        red_remain: 6,
        blue_remain: 6,
        data: [
            [1, 2, 3, 0, 0],
            [4, 5, 0, 0, 0],
            [6, 0, 0, 0, -6],
            [0 ,0, -5, 0, -4],
            [0, 0, -3, -2, -1],
        ]
    }, board);

    board.mov(3, 2, 2);
    board.mov(2, 2, 1);

    assert_eq!(board::Board {
        red_remain: 5,
        blue_remain: 6,
        data: [
            [1, 2, 3, 0, 0],
            [4, -5, 0, 0, 0],
            [6, 0, 0, 0, -6],
            [0 ,0, 0, 0, -4],
            [0, 0, -3, -2, -1],
        ]
    }, board);

    board.mov(0, 0, 4);
    assert_eq!(board::Board {
        red_remain: 4,
        blue_remain: 6,
        data: [
            [0, 1, 3, 0, 0],
            [4, -5, 0, 0, 0],
            [6, 0, 0, 0, -6],
            [0 ,0, 0, 0, -4],
            [0, 0, -3, -2, -1],
        ]
    }, board);
}

#[test]
fn get() {
    let board = board::Board::new();
    assert_eq!(1, board.get(0, 0));
    assert_eq!(2, board.get(0, 1));
    assert_eq!(0, board.get(2, 2));
    assert_eq!(-3, board.get(4, 2));
}

#[test]
fn set() {
    let mut board = board::Board::new();
    board.set(0, 0, 2);
    assert_eq!(board::Board {
        red_remain: 6,
        blue_remain: 6,
        data: [
            [2, 2, 3, 0, 0],
            [4, 5, 0, 0, 0],
            [6, 0, 0, 0, -6],
            [0 ,0, 0, -5, -4],
            [0, 0, -3, -2, -1],
        ]
    }, board);

    board.set(2, 2, 7);
    assert_eq!(board::Board {
        red_remain: 6,
        blue_remain: 6,
        data: [
            [2, 2, 3, 0, 0],
            [4, 5, 0, 0, 0],
            [6, 0, 7, 0, -6],
            [0 ,0, 0, -5, -4],
            [0, 0, -3, -2, -1],
        ]
    }, board);
}

#[test]
fn get_blue_remain() {
    let mut board = board::Board::new();
    board.mov(3, 3, 1);
    board.mov(2, 2, 1);

    assert_eq!(6, board.get_blue_remain());

    board.mov(0, 0, 5);
    assert_eq!(5, board.get_blue_remain());
}

#[test]
fn get_red_remain() {
    let mut board = board::Board::new();
    board.mov(3, 3, 1);
    board.mov(2, 2, 1);

    assert_eq!(5, board.get_red_remain());

    board.mov(0, 0, 5);
    assert_eq!(5, board.get_red_remain());

    board.mov(0, 1, 4);
    assert_eq!(4, board.get_red_remain());
}

#[test]
fn winner() {
    let board = board::Board::new();
    assert_eq!(0, board::winner(&board));

    let board = board::Board {
        red_remain: 4,
        blue_remain: 6,
        data: [
            [-5,  2,  3,  0,  0],
            [4,  0,  0,  0,  0],
            [6,  0,  0,  0, -6],
            [0,  0,  0, 0, -4],
            [0,  0, -3, -2, -1]
        ]
    };
    assert_eq!(2, board::winner(&board));
    
    let board = board::Board {
        red_remain: 0,
        blue_remain: 6,
        data: [
            [-5,  0,  0,  0,  0],
            [0,  0,  0,  0,  0],
            [0,  0,  0,  0, -6],
            [0,  0,  0, 0, -4],
            [0,  0, -3, -2, -1]
        ]
    };
    assert_eq!(2, board::winner(&board));

    let board = board::Board {
        red_remain: 2,
        blue_remain: 5,
        data: [
            [1,  0,  0,  0,  0],
            [0,  -5,  0,  0,  0],
            [0,  0,  0,  0, -6],
            [0,  0,  0, 0, -4],
            [0,  0, -3, -2, 5]
        ]
    };
    assert_eq!(1, board::winner(&board));
}