use rand::{rngs::ThreadRng, Rng};

pub struct Game {
    pub(crate) board: [[Option<u32>; 4]; 4],
    score: u32,
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        Self {
            score: 0,
            board: [[None; 4]; 4],
            rng: rand::thread_rng(),
        }
    }

    pub fn start(&mut self) {
        for _ in 0..2 {
            self.set_rand();
        }
    }

    fn set_rand(&mut self) -> Option<(usize, usize)> {
        let available: Vec<usize> = (0..16)
            .filter(|i| self.board[i / 4][i % 4].is_none())
            .collect();
        let len = available.len();
        if len == 0 {
            return None;
        }
        let index = available[self.rng.gen_range(0..len)];
        let new_val = if self.rng.gen::<f64>() > 0.7 { 4 } else { 2 };
        let (r, c) = (index / 4, index % 4);
        self.board[r][c] = Some(new_val);
        Some((r, c))
    }

    pub fn move_board(&mut self, direction: Direction) -> GameStatus {
        let (max_tile, changed) = match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        };
        if max_tile == 2048 {
            return GameStatus::Win;
        }
        if self.check_is_end() {
            return GameStatus::Lost;
        }
        if changed {
            self.set_rand();
        }
        GameStatus::Continue
    }

    pub fn board_to_string(&self) -> String {
        // game board display:
        // ┌──────┬──────┬──────┬──────┐
        // │      │      │      │      │
        // │    2 │ 1024 │    8 │   16 │
        // │      │      │      │      │
        // ├──────┼──────┼──────┼──────┤
        // │      │      │      │      │
        // │    2 │ 1024 │    8 │   16 │
        // │      │      │      │      │
        // ├──────┼──────┼──────┼──────┤
        // │      │      │      │      │
        // │    2 │ 1024 │    8 │   16 │
        // │      │      │      │      │
        // ├──────┼──────┼──────┼──────┤
        // │      │      │      │      │
        // │    2 │ 1024 │    8 │   16 │
        // │      │      │      │      │
        // └──────┴──────┴──────┴──────┘
        // height 17; width 31 (29 characters + \n\r)

        let mut output = String::with_capacity(17 * 31);
        output.push_str("┌──────┬──────┬──────┬──────┐\n\r");

        for row in 0..15 {
            if row % 2 == 0 {
                output.push_str("│      │      │      │      │");
            } else if (row - 1) % 4 == 0 {
                let board_row = (row - 1) / 4;
                output.push('│');
                for col in 0..4 {
                    match self.board[board_row][col] {
                        Some(val) => output.push_str(&format!("{:>5} │", val)[..]),
                        None => output.push_str("      │"),
                    };
                }
            } else {
                output.push_str("├──────┼──────┼──────┼──────┤");
            }
            output.push_str("\n\r");
        }
        output.push_str("└──────┴──────┴──────┴──────┘\n\r");

        output
    }

    fn check_is_end(&self) -> bool {
        !self.board.iter().enumerate().any(|(r, row)| {
            row.iter().enumerate().any(|(c, val)| {
                let next_row = if r + 1 < 4 {
                    self.board[r + 1][c]
                } else {
                    None
                };
                let next_col = if c + 1 < 4 {
                    self.board[r][c + 1]
                } else {
                    None
                };
                val.is_none() || *val == next_row || *val == next_col
            })
        })
    }

    pub(crate) fn move_up(&mut self) -> (u32, bool) {
        let mut max_tile: u32 = 0;
        let mut changed = false;
        for c in 0..4 {
            let mut start: usize = 0;
            let mut merged = false;
            for r in 0..4 {
                if let Some(val) = self.board[r][c].take() {
                    if start == 0 || Some(val) != self.board[start - 1][c] || merged {
                        max_tile = max_tile.max(val);
                        if start != r {
                            changed = true;
                        }
                        self.board[start][c] = Some(val);
                        start += 1;
                        merged = false;
                    } else {
                        let new_val = val + val;
                        max_tile = max_tile.max(new_val);
                        self.board[start - 1][c] = Some(new_val);
                        self.score += new_val;
                        merged = true;
                        changed = true;
                    }
                }
            }
        }
        (max_tile, changed)
    }

    pub(crate) fn move_down(&mut self) -> (u32, bool) {
        let mut max_tile: u32 = 0;
        let mut changed = false;
        for c in 0..4 {
            let mut start: isize = 3;
            let mut merged = false;
            for r in (0..4).rev() {
                if let Some(val) = self.board[r][c].take() {
                    if start == 3 || Some(val) != self.board[start as usize + 1][c] || merged {
                        max_tile = max_tile.max(val);
                        if start as usize != r {
                            changed = true;
                        }
                        self.board[start as usize][c] = Some(val);
                        start -= 1;
                        merged = false;
                    } else {
                        let new_val = val + val;
                        max_tile = max_tile.max(new_val);
                        self.board[start as usize + 1][c] = Some(new_val);
                        self.score += new_val;
                        merged = true;
                        changed = true;
                    }
                }
            }
        }
        (max_tile, changed)
    }

    pub(crate) fn move_left(&mut self) -> (u32, bool) {
        let mut max_tile: u32 = 0;
        let mut changed = false;
        for row in self.board.iter_mut() {
            let mut start: usize = 0;
            let mut merged = false;
            for i in 0..4 {
                if let Some(val) = row[i].take() {
                    if start == 0 || Some(val) != row[start - 1] || merged {
                        max_tile = max_tile.max(val);
                        if start != i {
                            changed = true;
                        }
                        row[start] = Some(val);
                        start += 1;
                        merged = false;
                    } else {
                        let new_val = val + val;
                        max_tile = max_tile.max(new_val);
                        row[start - 1] = Some(new_val);
                        self.score += new_val;
                        merged = true;
                        changed = true;
                    }
                }
            }
        }
        (max_tile, changed)
    }

    pub(crate) fn move_right(&mut self) -> (u32, bool) {
        let mut max_tile: u32 = 0;
        let mut changed = false;
        for row in self.board.iter_mut() {
            let mut start: isize = 3;
            let mut merged = false;
            for i in (0..4).rev() {
                if let Some(val) = row[i].take() {
                    if start == 3 || Some(val) != row[start as usize + 1] || merged {
                        max_tile = max_tile.max(val);
                        if start as usize != i {
                            changed = true;
                        }
                        row[start as usize] = Some(val);
                        start -= 1;
                        merged = false;
                    } else {
                        let new_val = val + val;
                        max_tile = max_tile.max(new_val);
                        row[start as usize + 1] = Some(new_val);
                        self.score += new_val;
                        merged = true;
                        changed = true;
                    }
                }
            }
        }
        (max_tile, changed)
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum GameStatus {
    Win,
    Lost,
    Continue,
}
