pub struct Game {
    pub(crate) board: [[Option<u32>; 4]; 4],
    // pub score: usize,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[None; 4]; 4],
        }
    }

    pub fn move_board(&mut self, direction: Direction) {
        match direction {
            Direction::UP => self.move_up(),
            Direction::DOWN => self.move_down(),
            Direction::LEFT => self.move_left(),
            Direction::RIGHT => self.move_right(),
        }
    }

    fn move_up(&mut self) {
        for c in 0..4 {
            let mut start: usize = 0;
            let mut merged = false;
            for r in 0..4 {
                self.board[r][c].take().map(|val| {
                    if start == 0 || Some(val) != self.board[start - 1][c] {
                        self.board[start][c] = Some(val);
                        start += 1;
                        merged = false;
                    } else {
                        if merged {
                            self.board[start][c] = Some(val);
                            start += 1;
                            merged = false;
                        } else {
                            self.board[start - 1][c] = Some(val + val);
                            merged = true;
                        }
                    }
                });
            }
        }
    }

    fn move_down(&mut self) {
        for c in 0..4 {
            let mut start: isize = 3;
            let mut merged = false;
            for r in (0..4).rev() {
                self.board[r][c].take().map(|val| {
                    if start == 3 || Some(val) != self.board[start as usize + 1][c] {
                        self.board[start as usize][c] = Some(val);
                        start -= 1;
                        merged = false;
                    } else {
                        if merged {
                            self.board[start as usize][c] = Some(val);
                            start -= 1;
                            merged = false;
                        } else {
                            self.board[start as usize + 1][c] = Some(val + val);
                            merged = true;
                        }
                    }
                });
            }
        }
    }

    fn move_left(&mut self) {
        for row in self.board.iter_mut() {
            let mut start: usize = 0;
            let mut merged = false;
            for i in 0..4 {
                row[i].take().map(|val| {
                    if start == 0 || Some(val) != row[start - 1] {
                        row[start] = Some(val);
                        start += 1;
                        merged = false;
                    } else {
                        if merged {
                            row[start] = Some(val);
                            start += 1;
                            merged = false;
                        } else {
                            row[start - 1] = Some(val + val);
                            merged = true;
                        }
                    }
                });
            }
        }
    }

    fn move_right(&mut self) {
        for row in self.board.iter_mut() {
            let mut start: isize = 3;
            let mut merged = false;
            for i in (0..4).rev() {
                row[i].take().map(|val| {
                    if start == 3 || Some(val) != row[start as usize + 1] {
                        row[start as usize] = Some(val);
                        start -= 1;
                        merged = false;
                    } else {
                        if merged {
                            row[start as usize] = Some(val);
                            start -= 1;
                            merged = false;
                        } else {
                            row[start as usize + 1] = Some(val + val);
                            merged = true;
                        }
                    }
                });
            }
        }
    }
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
