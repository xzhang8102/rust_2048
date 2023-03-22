use rand::{rngs::ThreadRng, Rng};

pub struct Game {
    score: u32,
    pub(crate) board: [[Option<u32>; 4]; 4],
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

    fn set_rand(&mut self) {
        let available: Vec<usize> = (0..16)
            .filter(|i| self.board[i / 4][i % 4].is_none())
            .collect();
        let len = available.len();
        if len == 0 {
            return;
        }
        let index = available[self.rng.gen_range(0..len)];
        let new_val = if self.rng.gen::<f64>() > 0.7 { 4 } else { 2 };
        self.board[index / 4][index % 4] = Some(new_val);
    }

    pub fn move_board(&mut self, direction: Direction) {
        let max_tile = match direction {
            Direction::UP => self.move_up(),
            Direction::DOWN => self.move_down(),
            Direction::LEFT => self.move_left(),
            Direction::RIGHT => self.move_right(),
        };
        if max_tile == 2048 {
            todo!("player has won");
        }
        if self.check_is_end() {
            todo!("player has lost");
        }
        self.set_rand();
    }

    fn check_is_end(&self) -> bool {
        self.board.iter().enumerate().any(|(r, row)| {
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

    pub(crate) fn move_up(&mut self) -> u32 {
        let mut max_tile: u32 = 0;
        for c in 0..4 {
            let mut start: usize = 0;
            let mut merged = false;
            for r in 0..4 {
                if let Some(val) = self.board[r][c].take() {
                    if start == 0 || Some(val) != self.board[start - 1][c] || merged {
                        max_tile = max_tile.max(val);
                        self.board[start][c] = Some(val);
                        start += 1;
                        merged = false;
                    } else {
                        let new_val = val + val;
                        max_tile = max_tile.max(new_val);
                        self.board[start - 1][c] = Some(new_val);
                        self.score += new_val;
                        merged = true;
                    }
                }
            }
        }
        max_tile
    }

    pub(crate) fn move_down(&mut self) -> u32 {
        let mut max_tile: u32 = 0;
        for c in 0..4 {
            let mut start: isize = 3;
            let mut merged = false;
            for r in (0..4).rev() {
                if let Some(val) = self.board[r][c].take() {
                    if start == 3 || Some(val) != self.board[start as usize + 1][c] || merged {
                        max_tile = max_tile.max(val);
                        self.board[start as usize][c] = Some(val);
                        start -= 1;
                        merged = false;
                    } else {
                        let new_val = val + val;
                        max_tile = max_tile.max(new_val);
                        self.board[start as usize + 1][c] = Some(new_val);
                        self.score += new_val;
                        merged = true;
                    }
                }
            }
        }
        max_tile
    }

    pub(crate) fn move_left(&mut self) -> u32 {
        let mut max_tile: u32 = 0;
        for row in self.board.iter_mut() {
            let mut start: usize = 0;
            let mut merged = false;
            for i in 0..4 {
                if let Some(val) = row[i].take() {
                    if start == 0 || Some(val) != row[start - 1] || merged {
                        max_tile = max_tile.max(val);
                        row[start] = Some(val);
                        start += 1;
                        merged = false;
                    } else {
                        let new_val = val + val;
                        max_tile = max_tile.max(new_val);
                        row[start - 1] = Some(new_val);
                        self.score += new_val;
                        merged = true;
                    }
                }
            }
        }
        max_tile
    }

    pub(crate) fn move_right(&mut self) -> u32 {
        let mut max_tile: u32 = 0;
        for row in self.board.iter_mut() {
            let mut start: isize = 3;
            let mut merged = false;
            for i in (0..4).rev() {
                if let Some(val) = row[i].take() {
                    if start == 3 || Some(val) != row[start as usize + 1] || merged {
                        max_tile = max_tile.max(val);
                        row[start as usize] = Some(val);
                        start -= 1;
                        merged = false;
                    } else {
                        let new_val = val + val;
                        max_tile = max_tile.max(new_val);
                        row[start as usize + 1] = Some(new_val);
                        self.score += new_val;
                        merged = true;
                    }
                }
            }
        }
        max_tile
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
