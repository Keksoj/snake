// 2019-06-15

use std::io::stdout;
mod game;
use game::Game;
mod cell;
mod snake;
extern crate termion;
use std::{thread, time};
use termion::async_stdin;

fn main() {
    // Get the standard input stream
    // termion::async_stdin() doesn't block the loop waiting for a key type
    let stdin = async_stdin();
    let stdout = stdout();

    println!("IJKL pour déplacer le serpent, Q pour quitter");
    thread::sleep(time::Duration::from_millis(2000));

    let mut game = Game::new(stdin, stdout.lock());

    game.run();
}
