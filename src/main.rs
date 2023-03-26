use std::io::{self, Write};

use game::{Direction, Game, GameStatus};
use termion::{clear, cursor, event::Key::Char, input::TermRead, raw::IntoRawMode, style};

mod game;
mod tests;

fn main() -> io::Result<()> {
    let stdout = io::stdout().lock();
    let mut stdout = stdout.into_raw_mode()?;
    write!(stdout, "{}", clear::All)?;
    write!(stdout, "{}", cursor::Goto(1, 1))?;

    let mut game = Game::new();
    game.start();
    stdout.write_all(game.board_to_string().as_bytes())?;
    stdout.flush().unwrap();
    let stdin = io::stdin().lock();
    let mut keys = stdin.keys();
    loop {
        let op = keys.next().unwrap().unwrap();
        let status = match op {
            Char('h') => game.move_board(Direction::Left),
            Char('j') => game.move_board(Direction::Down),
            Char('k') => game.move_board(Direction::Up),
            Char('l') => game.move_board(Direction::Right),
            Char('q') => break,
            _ => GameStatus::Continue,
        };
        match status {
            GameStatus::Win | GameStatus::Lost => {
                // TODO: game is terminated
                break;
            }
            GameStatus::Continue => {
                write!(stdout, "{}", cursor::Goto(1, 1))?;
                stdout.write_all(game.board_to_string().as_bytes())?;
                stdout.flush()?;
                continue;
            }
        };
    }

    write!(stdout, "{}", style::Reset)?;
    stdout.flush()?;
    Ok(())
}
