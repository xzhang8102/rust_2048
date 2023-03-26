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
    assert_eq!(game.move_left(), (4, true));
    assert_eq!(game.board[0], [Some(4), Some(4), None, None]);

    set_row(&mut game, 0, [Some(2), None, None, Some(2)]);
    assert_eq!(game.move_left(), (4, true));
    assert_eq!(game.board[0], [Some(4), None, None, None]);

    set_row(&mut game, 0, [Some(2), Some(2), Some(4), Some(4)]);
    assert_eq!(game.move_left(), (8, true));
    assert_eq!(game.board[0], [Some(4), Some(8), None, None]);

    set_row(&mut game, 0, [Some(2), None, Some(2), Some(4)]);
    assert_eq!(game.move_left(), (4, true));
    assert_eq!(game.board[0], [Some(4), Some(4), None, None]);

    set_row(&mut game, 0, [Some(2), Some(4), Some(8), Some(16)]);
    assert_eq!(game.move_left(), (16, false));
    assert_eq!(game.board[0], [Some(2), Some(4), Some(8), Some(16)]);
}

#[test]
fn move_right() {
    let mut game = Game::new();

    set_row(&mut game, 0, [Some(2), Some(2), Some(2), Some(2)]);
    assert_eq!(game.move_right(), (4, true));
    assert_eq!(game.board[0], [None, None, Some(4), Some(4)]);

    set_row(&mut game, 0, [Some(2), None, None, Some(2)]);
    assert_eq!(game.move_right(), (4, true));
    assert_eq!(game.board[0], [None, None, None, Some(4)]);

    set_row(&mut game, 0, [Some(4), Some(4), Some(2), Some(2)]);
    assert_eq!(game.move_right(), (8, true));
    assert_eq!(game.board[0], [None, None, Some(8), Some(4)]);

    set_row(&mut game, 0, [Some(2), None, Some(2), Some(4)]);
    assert_eq!(game.move_right(), (4, true));
    assert_eq!(game.board[0], [None, None, Some(4), Some(4)]);

    set_row(&mut game, 0, [Some(2), Some(4), Some(8), Some(16)]);
    assert_eq!(game.move_right(), (16, false));
    assert_eq!(game.board[0], [Some(2), Some(4), Some(8), Some(16)]);
}

#[test]
fn move_up() {
    let mut game = Game::new();

    set_col(&mut game, 0, [Some(2), Some(2), Some(2), Some(2)]);
    assert_eq!(game.move_up(), (4, true));
    assert_eq!(get_col(&mut game, 0), [Some(4), Some(4), None, None]);

    set_col(&mut game, 0, [Some(2), None, None, Some(2)]);
    assert_eq!(game.move_up(), (4, true));
    assert_eq!(get_col(&mut game, 0), [Some(4), None, None, None]);

    set_col(&mut game, 0, [Some(2), Some(2), Some(4), Some(4)]);
    assert_eq!(game.move_up(), (8, true));
    assert_eq!(get_col(&mut game, 0), [Some(4), Some(8), None, None]);

    set_col(&mut game, 0, [Some(2), None, Some(2), Some(4)]);
    assert_eq!(game.move_up(), (4, true));
    assert_eq!(get_col(&mut game, 0), [Some(4), Some(4), None, None]);

    set_col(&mut game, 0, [Some(2), Some(4), Some(8), Some(16)]);
    assert_eq!(game.move_up(), (16, false));
    assert_eq!(get_col(&mut game, 0), [Some(2), Some(4), Some(8), Some(16)]);
}

#[test]
fn move_down() {
    let mut game = Game::new();

    set_col(&mut game, 0, [Some(2), Some(2), Some(2), Some(2)]);
    assert_eq!(game.move_down(), (4, true));
    assert_eq!(get_col(&mut game, 0), [None, None, Some(4), Some(4)]);

    set_col(&mut game, 0, [Some(2), None, None, Some(2)]);
    assert_eq!(game.move_down(), (4, true));
    assert_eq!(get_col(&mut game, 0), [None, None, None, Some(4)]);

    set_col(&mut game, 0, [Some(4), Some(4), Some(2), Some(2)]);
    assert_eq!(game.move_down(), (8, true));
    assert_eq!(get_col(&mut game, 0), [None, None, Some(8), Some(4)]);

    set_col(&mut game, 0, [Some(2), None, Some(2), Some(4)]);
    assert_eq!(game.move_down(), (4, true));
    assert_eq!(get_col(&mut game, 0), [None, None, Some(4), Some(4)]);

    set_col(&mut game, 0, [Some(2), Some(4), Some(8), Some(16)]);
    assert_eq!(game.move_down(), (16, false));
    assert_eq!(get_col(&mut game, 0), [Some(2), Some(4), Some(8), Some(16)]);
}
