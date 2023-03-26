use rand::{rngs::ThreadRng, Rng};
use std::io::{self, prelude::*};
use termion::{clear, color, cursor, event::Key::Char, input::TermRead, raw::IntoRawMode, style};

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

    pub fn start(&mut self) -> io::Result<()> {
        let stdout = io::stdout().lock();
        let mut stdout = stdout.into_raw_mode()?;
        let stdin = io::stdin().lock();
        let mut stdin_keys = stdin.keys();

        for _ in 0..2 {
            self.set_rand();
        }

        write!(stdout, "{}", clear::All)?;
        write!(stdout, "{}", cursor::Goto(1, 1))?;
        stdout.write_all(self.board_to_string(None).as_bytes())?;
        stdout.write_all(
            format!("\n{}Score: {}{}\n\r", style::Bold, self.score, style::Reset).as_bytes(),
        )?;
        stdout.flush().unwrap();

        loop {
            let op = stdin_keys.next().unwrap().unwrap();
            let (status, coord) = match op {
                Char('h') => self.move_board(Direction::Left),
                Char('j') => self.move_board(Direction::Down),
                Char('k') => self.move_board(Direction::Up),
                Char('l') => self.move_board(Direction::Right),
                Char('q') => break,
                _ => (GameStatus::Continue, None),
            };
            match status {
                GameStatus::Win => {
                    write!(stdout, "{}", cursor::Goto(1, 1))?;
                    stdout.write_all(self.board_to_string(coord).as_bytes())?;
                    stdout.write_all(
                        format!("\n{}Score: {}{}\n\r", style::Bold, self.score, style::Reset)
                            .as_bytes(),
                    )?;
                    stdout.flush()?;
                    break;
                }
                GameStatus::Lost => {
                    // TODO: game is terminated
                    break;
                }
                GameStatus::Continue => {
                    if coord.is_some() {
                        write!(stdout, "{}", cursor::Goto(1, 1))?;
                        stdout.write_all(self.board_to_string(coord).as_bytes())?;
                        stdout.write_all(
                            format!("\n{}Score: {}{}\n\r", style::Bold, self.score, style::Reset)
                                .as_bytes(),
                        )?;
                        stdout.flush()?;
                    }
                    continue;
                }
            };
        }

        write!(stdout, "{}", style::Reset)?;
        stdout.flush()?;
        Ok(())
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

    pub fn move_board(&mut self, direction: Direction) -> (GameStatus, Option<(usize, usize)>) {
        let (max_tile, changed) = match direction {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        };
        if max_tile == 2048 {
            return (GameStatus::Win, None);
        }
        if self.check_is_end() {
            return (GameStatus::Lost, None);
        }
        if changed {
            return (GameStatus::Continue, self.set_rand());
        }
        (GameStatus::Continue, None)
    }

    pub fn board_to_string(&self, coord: Option<(usize, usize)>) -> String {
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
                        Some(val) => match coord {
                            Some((r, c)) if r == board_row && c == col => output.push_str(
                                &format!("{}{:>5}{} │", style::Bold, val, style::Reset)[..],
                            ),
                            _ => {
                                let palette = match val {
                                    8 => color::Rgb(0xff, 0xe4, 0xe6),
                                    16 => color::Rgb(0xfe, 0xcd, 0xd3),
                                    32 => color::Rgb(0xfd, 0xa4, 0xaf),
                                    64 => color::Rgb(0xfb, 0x71, 0x85),
                                    128 => color::Rgb(0xf4, 0x3f, 0x5e),
                                    256 => color::Rgb(0xe1, 0x1d, 0x48),
                                    512 => color::Rgb(0xbe, 0x12, 0x3c),
                                    1024 => color::Rgb(0x9f, 0x12, 0x39),
                                    2048 => color::Rgb(0x88, 0x13, 0x37),
                                    _ => color::Rgb(0xff, 0xf1, 0xf2),
                                };
                                let color_str =
                                    format!("{}{:>5}{} │", color::Fg(palette), val, style::Reset);
                                output.push_str(&color_str[..]);
                            }
                        },
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
