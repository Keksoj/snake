use crate::cell;
use crate::cell::Cell;
use crate::snake::{Snake, Turning};
use rand::Rng;
use std::io::{Read, Write};
use std::{thread, time};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::terminal_size;
use termion::{clear, cursor};

// Game is a struct that contains all information about the game
pub struct Game<R, W: Write> {
    width: usize,
    height: usize,
    board: Vec<Cell>, // There are no (x, y) coordinates, only one axis that gets
    // chunked, like this:
    //  0  1  2  3  4  5  6  7  8  9 10 11
    // 12 13 14 15 16 17 18 19 20 21 22 23
    // 24 25 26 27 28 29 30 31 32 33 34 35
    // etc.
    // each Cell of the grid is an enum,
    // either empty, snake body, or food
    food: usize,  // the food is one single coordinate
    snake: Snake, // The snake is a vector of coordinates
    stdout: W,
    stdin: R,
    tick_time: u64, // milliseconds of thread sleep at each loop
}

impl<R: Read, W: Write> Game<R, W> {
    // Set the new game to fit the terminal size.
    pub fn new(stdin: R, stdout: W) -> Game<R, RawTerminal<W>> {
        let mut new_game = Game {
            width: (terminal_size().unwrap().0 as usize) - 3,
            height: (terminal_size().unwrap().1 as usize) - 4,
            board: Vec::new(),
            snake: Snake::new(),
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            food: 50,
            tick_time: 300,
        };

        // Fill the board with enough items
        let total_length = new_game.width * new_game.height;
        for _i in 0..total_length {
            new_game.board.push(Cell::Empty)
        }

        new_game
    }

    pub fn run(&mut self) {
        loop {
            self.take_direction();
            self.snake.advance(self.width);
            self.check_for_food();
            self.check_for_collisions();
            self.write_snake_and_food_on_the_board();
            self.display_board();
            thread::sleep(time::Duration::from_millis(self.tick_time));
        }
    }

    pub fn take_direction(&mut self) {
        match self.snake.direction {
            Direction::Up | Direction::Down => self.snake.turning = Turning::Keepvertical,
            Direction::Left | Direction::Right => self.snake.turning = Turning::Keephorizontal,
        }

        // read a single byte from stdin
        let mut b = [0];
        self.stdin.read(&mut b).unwrap();
        match b[0] {
            b'i' => self.snake.take_direction(Direction::Up),
            b'j' => self.snake.take_direction(Direction::Left),
            b'l' => self.snake.take_direction(Direction::Right),
            b'k' => self.snake.take_direction(Direction::Down),
            b'q' => panic!("c'est la panique !"),
            _ => {}
        }
        self.stdout.flush().unwrap();
    }

    fn check_for_collisions(&mut self) {
        let snake_head = self.snake.head_coordinate();

        if snake_head % self.width == 0 || snake_head > self.width * self.height
        || self.snake.body_collides_with(snake_head)
        {
            self.game_over("Collision. Game over!".to_string())
        }
    }

    pub fn check_for_food(&mut self) {
        if self.snake.head_coordinate() == self.food {
            let mut new_food_position = rand::thread_rng().gen_range(1, self.width * self.height);
            while self.snake.body_collides_with(new_food_position) {
                new_food_position = rand::thread_rng().gen_range(1, self.width * self.height);
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

    pub fn display_board(&mut self) {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        // the top wall
        self.stdout.write(cell::TOP_LEFT_CORNER.as_bytes()).unwrap();
        for _n in 0..self.width {
            self.stdout.write(cell::CEILING.as_bytes()).unwrap();
        }
        self.stdout
            .write(cell::TOP_RIGHT_CORNER.as_bytes())
            .unwrap();
        self.stdout.write(b"\n\r").unwrap();

        // display each line
        for line in self.board.as_slice().chunks(self.width) {
            self.stdout.write(cell::WALL.as_bytes()).unwrap();
            for &cell in line {
                self.stdout
                    .write(cell.match_to_symbol().as_bytes())
                    .unwrap();
            }
            self.stdout.write(cell::WALL.as_bytes()).unwrap();
            self.stdout.write(b"\n\r").unwrap();
        }
        // the bottom wall
        self.stdout
            .write(cell::BOTTOM_LEFT_CORNER.as_bytes())
            .unwrap();
        for _n in 0..self.width {
            self.stdout.write(cell::CEILING.as_bytes()).unwrap();
        }
        self.stdout
            .write(cell::BOTTOM_RIGHT_CORNER.as_bytes())
            .unwrap();
        self.stdout.write(b"\n\r").unwrap();

        // Some nasty way of displaying the score (the snake's length)
        self.stdout.write(b"\n\rYour score: ").unwrap();
        let snake_length_string: String = self.snake.body.len().to_string();
        let snake_position_bytes: &[u8] = snake_length_string.as_bytes();
        self.stdout.write(snake_position_bytes).unwrap();
        self.stdout.flush().unwrap();
        // write!(self.stdout, "{}", cursor::Hide).unwrap();
    }

    fn game_over(&mut self, message: String) {
        panic!(message);
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}
