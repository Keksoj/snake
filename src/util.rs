use crate::graphics;
use rand::Rng;
use std::io::{Read, Write};
use std::{thread, time};
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::terminal_size;
use termion::{clear, cursor};

// Game is a struct that contains all information about the game
pub struct Game<R, W: Write> {
    width: u32,
    height: u32,
    board: Vec<Cell>, // There are no (x, y) coordinates, only one axis that gets
    // chunked, like this:
    //  0  1  2  3  4  5  6  7  8  9 10 11
    // 12 13 14 15 16 17 18 19 20 21 22 23
    // 24 25 26 27 28 29 30 31 32 33 34 35
    // etc.
    // each Cell of the grid is an enum,
    // either empty, snake body, or food
    food: u32,       // the food is one single coordinate
    snake: Vec<u32>, // The snake is a vector of coordinates
    stdout: W,
    stdin: R,
    direction: Direction, // an enum, either Left, Right, Up, Down
    turning: Turning,     // an enum as well, usefull for drawing the snake
    counter: u8,          // used for upping the snake's size and speed
    speed: u64,           // milliseconds of thread sleep at each loop
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Food,
    Head,
    Tail,
    Uptoleft,
    Uptoright,
    Downtoleft,
    Downtoright,
    Horizontal,
    Vertical,
}

impl<R: Read, W: Write> Game<R, W> {
    // Set the new game to fit the terminal size.
    pub fn new(stdin: R, stdout: W) -> Game<R, RawTerminal<W>> {
        let mut new_game = Game {
            width: (terminal_size().unwrap().0 as u32) - 3,
            height: (terminal_size().unwrap().1 as u32) - 4,
            board: Vec::new(),
            snake: [0, 1, 2, 3, 4].to_vec(),
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            direction: Direction::Right,
            turning: Turning::Horizontal,
            food: 50,
            counter: 0,
            speed: 300,
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
            self.update_the_game();
            self.check_for_food();
            self.display_board();
            thread::sleep(time::Duration::from_millis(self.speed));
        }
    }

    pub fn take_direction(&mut self) {
        match self.direction {
            Direction::Up | Direction::Down => self.turning = Turning::Vertical,
            Direction::Left | Direction::Right => self.turning = Turning::Horizontal,
        }

        // read a single byte from stdin
        let mut b = [0];
        self.stdin.read(&mut b).unwrap();
        match b[0] {
            b'i' => self.turn_snake(Direction::Up),
            b'j' => self.turn_snake(Direction::Left),
            b'l' => self.turn_snake(Direction::Right),
            b'k' => self.turn_snake(Direction::Down),
            b'q' => panic!("c'est la panique !"),
            _ => {}
        }

        // check if the direction comes against the left wall
        if self.snake[self.snake.len() - 1] % self.width == 0 && self.direction == Direction::Left {
            panic!("T'es rentré dans le mur de gauche !")
        }
        self.stdout.flush().unwrap();
    }

    pub fn update_the_game(&mut self) {
        // set some new food
        if self.counter == 3 {
            // we must avoid putting the food in the snake's body
            let old_food = self.food;
            while self.food == old_food {
                let new_food = rand::thread_rng().gen_range(1, self.width * self.height);
                if !self.snake.contains(&new_food) {
                    self.food = new_food
                }
            }
        }
        // Push the new food in the board board
        self.board[self.food as usize] = Cell::Food;

        // Remove the tail IF the counter is at 3, 2 or 1
        if self.counter > 0 {
            self.counter = self.counter - 1
        } else {
            // Make the former tail disappear from the board...
            self.board[self.snake[0] as usize] = Cell::Empty;
            // ... and from the snake
            self.snake.remove(0);
        }

        // draw the new tail (verbose way)
        let new_tail = self.snake[0];
        self.board[new_tail as usize] = Cell::Tail;

        // find the index of the former_head and draw the snake body accordingly
        let former_head = self.snake[self.snake.len() - 1];
        match self.turning {
            Turning::Uptoleft => self.board[former_head as usize] = Cell::Uptoleft,
            Turning::Uptoright => self.board[former_head as usize] = Cell::Uptoright,
            Turning::Downtoleft => self.board[former_head as usize] = Cell::Downtoleft,
            Turning::Downtoright => self.board[former_head as usize] = Cell::Downtoright,
            Turning::Horizontal => self.board[former_head as usize] = Cell::Horizontal,
            Turning::Vertical => self.board[former_head as usize] = Cell::Vertical,
        }

        // Compute the new index of the snake's head depending of the direction
        let new_head: u32 = match self.direction {
            Direction::Left => former_head - 1,
            Direction::Right => former_head + 1,
            Direction::Up => former_head - self.width,
            Direction::Down => former_head + self.width,
        };

        // check for collisions against the snake and the walls
        let iterateur = self.snake.iter();
        for i in iterateur {
            if new_head == *i {
                panic!("Le serpent se mord la queue !")
            } else if new_head % self.width == 0 && self.direction == Direction::Right {
                panic!("On est rentrés dans le mur de droite !")
            } else if new_head > self.width * self.height {
                panic!("On est rentrés dans le mur du bas !")
            }
        }
        // Add the new head to the snake and draw it on the board
        self.snake.push(new_head);
        self.board[new_head as usize] = Cell::Head;
    }

    pub fn check_for_food(&mut self) {
        if self.snake[self.snake.len() - 1] == self.food {
            self.counter = self.counter + 3;
            self.speed = self.speed - 4
        } else {
            return;
        }
    }

    // Give the snake its new direction and take notice of the change (self.turning)
    pub fn turn_snake(&mut self, new_dir: Direction) {
        match (&self.direction, &new_dir) {
            // Prevent 180° turns
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => return,

            // Take notice of the direction change... so boilerplate
            (Direction::Up, Direction::Left) |
            (Direction::Right, Direction::Down) => {
                self.turning = Turning::Uptoleft;
                self.direction = new_dir
            }
            (Direction::Up, Direction::Right) |
            (Direction::Left, Direction::Down) => {
                self.turning = Turning::Uptoright;
                self.direction = new_dir
            }
            (Direction::Down, Direction::Left) |
            (Direction::Right, Direction::Up) => {
                self.turning = Turning::Downtoleft;
                self.direction = new_dir
            }
            (Direction::Down, Direction::Right) |
            (Direction::Left, Direction::Up) => {
                self.turning = Turning::Downtoright;
                self.direction = new_dir
            }
            _ => self.direction = new_dir,
        }
    }

    pub fn display_board(&mut self) {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        // the top wall
        self.stdout.write(graphics::TOP_LEFT_CORNER.as_bytes()).unwrap();
        for _n in 0..self.width {
            self.stdout.write(graphics::CEILING.as_bytes()).unwrap();
        }
        self.stdout.write(graphics::TOP_RIGHT_CORNER.as_bytes()).unwrap();
        self.stdout.write(b"\n\r").unwrap();

        // display each line
        for line in self.board.as_slice().chunks(self.width as usize) {
            self.stdout.write(graphics::WALL.as_bytes()).unwrap();
            for &cell in line {
                let symbol = match cell {
                    Cell::Empty => graphics::EMPTY,
                    Cell::Food => graphics::FOOD,
                    Cell::Head => match self.direction {
                                    Direction::Up => graphics::UP,
                                    Direction::Down => graphics::DOWN,
                                    Direction::Left => graphics::LEFT,
                                    Direction::Right => graphics::RIGHT,
                                    }
                    Cell::Tail => graphics::TAIL,
                    Cell::Uptoleft => graphics::UPTOLEFT,
                    Cell::Uptoright => graphics::UPTORIGHT,
                    Cell::Downtoleft => graphics::DOWNTOLEFT,
                    Cell::Downtoright => graphics::DOWNTORIGHT,
                    Cell::Horizontal => graphics::HORIZONTAL,
                    Cell::Vertical => graphics::VERTICAL,
                };
                self.stdout.write(symbol.as_bytes()).unwrap();
            }
            self.stdout.write(graphics::WALL.as_bytes()).unwrap();
            self.stdout.write(b"\n\r").unwrap();
        }

        // the bottom wall
        self.stdout.write(graphics::BOTTOM_LEFT_CORNER.as_bytes()).unwrap();
        for _n in 0..self.width {
            self.stdout.write(graphics::CEILING.as_bytes()).unwrap();
        }
        self.stdout.write(graphics::BOTTOM_RIGHT_CORNER.as_bytes()).unwrap();
        self.stdout.write(b"\n\r").unwrap();

        // Some nasty way of displaying the score (the snake's length)
        self.stdout.write(b"\n\rYour score: ").unwrap();
        let snake_length_string: String = self.snake.len().to_string();
        let snake_position_bytes: &[u8] = snake_length_string.as_bytes();
        self.stdout.write(snake_position_bytes).unwrap();

        self.stdout.flush().unwrap();
        // write!(self.stdout, "{}", cursor::Hide).unwrap();
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Debug, PartialEq)]
pub enum Turning {
    Uptoleft,
    Uptoright,
    Downtoleft,
    Downtoright,
    Horizontal,
    Vertical,
}
