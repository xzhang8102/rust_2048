use crate::game::{Direction, Game};

fn set_row(game: &mut Game, row_n: usize, row: [u32; 4]) {
    game.board[row_n] = row;
}

#[test]
fn move_left() {
    let mut game = Game::new();

    set_row(&mut game, 0, [2, 2, 2, 2]);
    game.move_board(Direction::LEFT);
    assert_eq!(game.board[0], [4, 4, 0, 0]);

    set_row(&mut game, 0, [2, 0, 0, 2]);
    game.move_board(Direction::LEFT);
    assert_eq!(game.board[0], [4, 0, 0, 0]);

    set_row(&mut game, 0, [2, 2, 4, 8]);
    game.move_board(Direction::LEFT);
    assert_eq!(game.board[0], [4, 4, 8, 0]);

    set_row(&mut game, 0, [2, 0, 2, 4]);
    game.move_board(Direction::LEFT);
    assert_eq!(game.board[0], [4, 4, 0, 0]);
}

#[test]
fn move_right() {
    let mut game = Game::new();

    set_row(&mut game, 0, [2, 2, 2, 2]);
    game.move_board(Direction::RIGHT);
    assert_eq!(game.board[0], [0, 0, 4, 4]);

    set_row(&mut game, 0, [2, 0, 0, 2]);
    game.move_board(Direction::RIGHT);
    assert_eq!(game.board[0], [0, 0, 0, 4]);

    set_row(&mut game, 0, [2, 2, 4, 8]);
    game.move_board(Direction::RIGHT);
    assert_eq!(game.board[0], [0, 4, 4, 8]);

    set_row(&mut game, 0, [2, 0, 2, 4]);
    game.move_board(Direction::RIGHT);
    assert_eq!(game.board[0], [0, 0, 4, 4]);
}
