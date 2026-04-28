use std::{
    io::{Error as IoError, Read, Write},
    os::fd::AsFd,
    thread, time,
};

use rand::random_range;
use termion::{
    raw::IntoRawMode,
    {clear, cursor, raw::RawTerminal, terminal_size},
};
use thiserror::Error;

use crate::{
    cell::{self, Cell},
    snake::{Snake, Turning},
};

#[derive(Error, Debug)]
pub enum GameError {
    #[error("terminal size error: {0}")]
    TerminalSize(IoError),
    #[error("could not switch terminal to raw mode: {0}")]
    TerminalRawMode(IoError),
    #[error("IO read error: {0}")]
    IoRead(IoError),
    #[error("Flush error: {0}")]
    Flush(IoError),
    #[error("Write error: {0}")]
    Write(IoError),
    #[error("Collision!")]
    Collision,
}

pub struct Game<R, W: Write> {
    width: usize,
    height: usize,
    board: Vec<Cell>,
    food: usize,
    snake: Snake,
    stdout: W,
    stdin: R,
    tick_time: u64,
}

impl<R: Read, W: Write + AsFd> Game<R, W> {
    pub fn new(stdin: R, stdout: W) -> Result<Game<R, RawTerminal<W>>, GameError> {
        let terminal_size = terminal_size().map_err(GameError::TerminalSize)?;
        let stdout = stdout.into_raw_mode().map_err(GameError::TerminalRawMode)?;

        let game = Game {
            width: terminal_size.0 as usize - 3,
            height: terminal_size.1 as usize - 4,
            board: Vec::new(),
            snake: Snake::new(),
            stdout,
            stdin,
            food: 50,
            tick_time: 300,
        };
        let mut new_game = game;

        // Fill the board with enough items
        let total_length = new_game.width * new_game.height;
        for _i in 0..total_length {
            new_game.board.push(Cell::Empty)
        }

        Ok(new_game)
    }

    pub fn run(&mut self) -> Result<(), GameError> {
        loop {
            self.take_direction()?;
            self.snake.advance(self.width);
            self.check_for_food();
            self.check_for_collisions()?;
            self.write_snake_and_food_on_the_board();
            self.display_board()?;
            thread::sleep(time::Duration::from_millis(self.tick_time));
        }
    }

    pub fn take_direction(&mut self) -> Result<(), GameError> {
        match self.snake.direction {
            Direction::Up | Direction::Down => self.snake.turning = Turning::Keepvertical,
            Direction::Left | Direction::Right => self.snake.turning = Turning::Keephorizontal,
        }

        let mut b = [0];
        self.stdin.read(&mut b).map_err(GameError::IoRead)?;
        match b[0] {
            b'i' => self.snake.take_direction(Direction::Up),
            b'j' => self.snake.take_direction(Direction::Left),
            b'l' => self.snake.take_direction(Direction::Right),
            b'k' => self.snake.take_direction(Direction::Down),
            b'q' => panic!("c'est la panique !"),
            _ => {}
        }
        self.stdout.flush().map_err(GameError::Flush)?;
        Ok(())
    }

    fn check_for_collisions(&mut self) -> Result<(), GameError> {
        let snake_head = self.snake.head_coordinate();

        if snake_head % self.width == 0
            || snake_head > self.width * self.height
            || self.snake.body_collides_with(snake_head)
        {
            return Err(GameError::Collision);
        }
        Ok(())
    }

    pub fn check_for_food(&mut self) {
        if self.snake.head_coordinate() == self.food {
            let mut new_food_position = random_range(1..self.width * self.height);
            while self.snake.body_collides_with(new_food_position) {
                new_food_position = random_range(1..self.width * self.height);
            }
            self.food = new_food_position;
            self.snake.set_growth_bonus(3);
            self.tick_time -= 4;
        }
    }

    pub fn write_snake_and_food_on_the_board(&mut self) {
        self.board = vec![Cell::Empty; self.width * self.height];
        self.board[self.food] = Cell::Food;
        for snake_cell in self.snake.body.iter() {
            self.board[snake_cell.0] = snake_cell.1
        }
    }

    pub fn display_board(&mut self) -> Result<(), GameError> {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).map_err(GameError::Write)?;

        // the top wall
        self.stdout
            .write(cell::TOP_LEFT_CORNER.as_bytes())
            .map_err(GameError::Write)?;

        for _n in 0..self.width {
            self.stdout
                .write(cell::CEILING.as_bytes())
                .map_err(GameError::Write)?;
        }
        self.stdout
            .write(cell::TOP_RIGHT_CORNER.as_bytes())
            .map_err(GameError::Write)?;

        self.stdout.write(b"\n\r").map_err(GameError::Write)?;

        // display each line
        for line in self.board.as_slice().chunks(self.width) {
            self.stdout
                .write(cell::WALL.as_bytes())
                .map_err(GameError::Write)?;

            for &cell in line {
                self.stdout
                    .write(cell.match_to_symbol().as_bytes())
                    .map_err(GameError::Write)?;
            }
            self.stdout
                .write(cell::WALL.as_bytes())
                .map_err(GameError::Write)?;
            self.stdout.write(b"\n\r").map_err(GameError::Write)?;
        }

        // the bottom wall
        self.stdout
            .write(cell::BOTTOM_LEFT_CORNER.as_bytes())
            .map_err(GameError::Write)?;
        for _n in 0..self.width {
            self.stdout
                .write(cell::CEILING.as_bytes())
                .map_err(GameError::Write)?;
        }
        self.stdout
            .write(cell::BOTTOM_RIGHT_CORNER.as_bytes())
            .map_err(GameError::Write)?;
        self.stdout.write(b"\n\r").map_err(GameError::Write)?;

        // score
        self.stdout
            .write(b"\n\rYour score: ")
            .map_err(GameError::Write)?;

        let snake_length_string: String = self.snake.body.len().to_string();
        let snake_position_bytes: &[u8] = snake_length_string.as_bytes();
        self.stdout
            .write(snake_position_bytes)
            .map_err(GameError::Write)?;

        self.stdout.flush().map_err(GameError::Flush)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}
