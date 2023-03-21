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
            let mut new_row: [u32; 4] = [0; 4];
            let mut merged = false;
            for &val in row.iter() {
                if val != 0 {
                    if start == 0 {
                        new_row[start] = val;
                        start += 1;
                        merged = false;
                    } else {
                        if val == new_row[start - 1] {
                            if merged {
                                new_row[start] = val;
                                start += 1;
                                merged = false;
                            } else {
                                new_row[start - 1] += val;
                                merged = true;
                            }
                        } else {
                            new_row[start] = val;
                            start += 1;
                            merged = false;
                        }
                    }
                }
            }
            *row = new_row;
        }
    }

    fn move_right(&mut self) {
        for row in self.board.iter_mut() {
            let mut start: usize = 3;
            let mut new_row: [u32; 4] = [0; 4];
            let mut merged = false;
            for &val in row.iter().rev() {
                if val != 0 {
                    if start == 3 {
                        new_row[start] = val;
                        start -= 1;
                        merged = false;
                    } else {
                        if val == new_row[start + 1] {
                            if merged {
                                new_row[start] = val;
                                start -= 1;
                                merged = false;
                            } else {
                                new_row[start + 1] += val;
                                merged = true;
                            }
                        } else {
                            new_row[start] = val;
                            start -= 1;
                            merged = false;
                        }
                    }
                }
            }
            *row = new_row;
        }
    }
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

