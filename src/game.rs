pub struct Game {
    pub(crate) board: [[u32; 4]; 4],
    // pub score: usize,
}

impl Game {
    pub fn new() -> Self {
        Self { board: [[0; 4]; 4] }
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
        todo!()
    }

    fn move_down(&mut self) {
        todo!()
    }

    fn move_left(&mut self) {
        for row in self.board.iter_mut() {
            let mut start: usize = 0;
            let mut merged = false;
            for i in 0..4 {
                let val = row[i];
                if val != 0 {
                    if start == 0 {
                        row[start] = val;
                        start += 1;
                        merged = false;
                    } else {
                        if val == row[start - 1] {
                            if merged {
                                row[start] = val;
                                start += 1;
                                merged = false;
                            } else {
                                row[start - 1] += val;
                                merged = true;
                            }
                        } else {
                            row[start] = val;
                            start += 1;
                            merged = false;
                        }
                    }
                }
            }
            if start <= 3 {
                row[start..4].fill(0);
            }
        }
    }

    fn move_right(&mut self) {
        for row in self.board.iter_mut() {
            let mut start: i8 = 3;
            let mut merged = false;
            for i in (0..4).rev() {
                let val = row[i];
                if val != 0 {
                    if start == 3 {
                        row[start as usize] = val;
                        start -= 1;
                        merged = false;
                    } else {
                        if val == row[start as usize + 1] {
                            if merged {
                                row[start as usize] = val;
                                start -= 1;
                                merged = false;
                            } else {
                                row[start as usize + 1] += val;
                                merged = true;
                            }
                        } else {
                            row[start as usize] = val;
                            start -= 1;
                            merged = false;
                        }
                    }
                }
            }
            if start >= 0 {
                row[0..=(start as usize)].fill(0);
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
