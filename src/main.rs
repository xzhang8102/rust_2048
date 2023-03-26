use std::io;
use game::Game;

mod game;
mod tests;

fn main() -> io::Result<()> {
    let mut game = Game::new();
    game.start()?;
    Ok(())
}
