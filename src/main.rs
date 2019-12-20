// 2019-06-15

use std::io::stdout;
mod util;
use util::Game;
mod graphics;
extern crate termion;
use termion::async_stdin;
use std::{thread, time};

fn main() {
    // Get the standard input stream
    // termion::async_stdin() doesn't block the loop waiting for a key type
    let stdin = async_stdin();
    let stdout = stdout();

    println!("Utilise IJKL pour déplacer le serpent ;-)");
    thread::sleep(time::Duration::from_millis(2000));

    let mut game = Game::new(stdin, stdout.lock());

    game.run();
}
