use crate::game::Game;

fn set_row(game: &mut Game, row_n: usize, row: [Option<u32>; 4]) {
    game.board[row_n] = row;
}

fn set_col(game: &mut Game, col_n: usize, col: [Option<u32>; 4]) {
    for (r, val) in col.into_iter().enumerate() {
        game.board[r][col_n] = val;
    }
}

fn get_col(game: &mut Game, col_n: usize) -> [Option<u32>; 4] {
    let mut result = [None; 4];
    for r in 0..4 {
        result[r] = game.board[r][col_n];
    }
    result
}

#[test]
fn move_left() {
    let mut game = Game::new();

    set_row(&mut game, 0, [Some(2), Some(2), Some(2), Some(2)]);
    game.move_left();
    assert_eq!(game.board[0], [Some(4), Some(4), None, None]);

    set_row(&mut game, 0, [Some(2), None, None, Some(2)]);
    game.move_left();
    assert_eq!(game.board[0], [Some(4), None, None, None]);

    set_row(&mut game, 0, [Some(2), Some(2), Some(4), Some(8)]);
    game.move_left();
    assert_eq!(game.board[0], [Some(4), Some(4), Some(8), None]);

    set_row(&mut game, 0, [Some(2), None, Some(2), Some(4)]);
    game.move_left();
    assert_eq!(game.board[0], [Some(4), Some(4), None, None]);

    set_row(&mut game, 0, [Some(2), Some(4), Some(8), Some(16)]);
    game.move_left();
    assert_eq!(game.board[0], [Some(2), Some(4), Some(8), Some(16)]);
}

#[test]
fn move_right() {
    let mut game = Game::new();

    set_row(&mut game, 0, [Some(2), Some(2), Some(2), Some(2)]);
    game.move_right();
    assert_eq!(game.board[0], [None, None, Some(4), Some(4)]);

    set_row(&mut game, 0, [Some(2), None, None, Some(2)]);
    game.move_right();
    assert_eq!(game.board[0], [None, None, None, Some(4)]);

    set_row(&mut game, 0, [Some(2), Some(2), Some(4), Some(8)]);
    game.move_right();
    assert_eq!(game.board[0], [None, Some(4), Some(4), Some(8)]);

    set_row(&mut game, 0, [Some(2), None, Some(2), Some(4)]);
    game.move_right();
    assert_eq!(game.board[0], [None, None, Some(4), Some(4)]);

    set_row(&mut game, 0, [Some(2), Some(4), Some(8), Some(16)]);
    game.move_right();
    assert_eq!(game.board[0], [Some(2), Some(4), Some(8), Some(16)]);
}

#[test]
fn move_up() {
    let mut game = Game::new();

    set_col(&mut game, 0, [Some(2), Some(2), Some(2), Some(2)]);
    game.move_up();
    assert_eq!(get_col(&mut game, 0), [Some(4), Some(4), None, None]);

    set_col(&mut game, 0, [Some(2), None, None, Some(2)]);
    game.move_up();
    assert_eq!(get_col(&mut game, 0), [Some(4), None, None, None]);

    set_col(&mut game, 0, [Some(2), Some(2), Some(4), Some(8)]);
    game.move_up();
    assert_eq!(get_col(&mut game, 0), [Some(4), Some(4), Some(8), None]);

    set_col(&mut game, 0, [Some(2), None, Some(2), Some(4)]);
    game.move_up();
    assert_eq!(get_col(&mut game, 0), [Some(4), Some(4), None, None]);

    set_col(&mut game, 0, [Some(2), Some(4), Some(8), Some(16)]);
    game.move_up();
    assert_eq!(get_col(&mut game, 0), [Some(2), Some(4), Some(8), Some(16)]);
}

#[test]
fn move_down() {
    let mut game = Game::new();

    set_col(&mut game, 0, [Some(2), Some(2), Some(2), Some(2)]);
    game.move_down();
    assert_eq!(get_col(&mut game, 0), [None, None, Some(4), Some(4)]);

    set_col(&mut game, 0, [Some(2), None, None, Some(2)]);
    game.move_down();
    assert_eq!(get_col(&mut game, 0), [None, None, None, Some(4)]);

    set_col(&mut game, 0, [Some(2), Some(2), Some(4), Some(8)]);
    game.move_down();
    assert_eq!(get_col(&mut game, 0), [None, Some(4), Some(4), Some(8)]);

    set_col(&mut game, 0, [Some(2), None, Some(2), Some(4)]);
    game.move_down();
    assert_eq!(get_col(&mut game, 0), [None, None, Some(4), Some(4)]);

    set_col(&mut game, 0, [Some(2), Some(4), Some(8), Some(16)]);
    game.move_down();
    assert_eq!(get_col(&mut game, 0), [Some(2), Some(4), Some(8), Some(16)]);
}
