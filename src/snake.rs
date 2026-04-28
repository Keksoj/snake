use crate::cell::Cell;
use crate::game::{Direction, GameError};

pub struct Snake {
    pub body: Vec<(usize, Cell)>,
    pub growth_counter: u8,
    pub direction: Direction,
    pub turning: Turning,
    pub head_symbol: Cell,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: [
                (3, Cell::Tail),
                (4, Cell::Horizontal),
                (5, Cell::Horizontal),
                (6, Cell::Horizontal),
                (7, Cell::Headsright),
            ]
            .to_vec(),
            growth_counter: 0,
            direction: Direction::Right,
            head_symbol: Cell::Headsright,
            turning: Turning::Keephorizontal,
        }
    }

    pub fn head_coordinate(&mut self) -> usize {
        self.body[self.body.len() - 1].0
    }
    pub fn set_growth_bonus(&mut self, bonus: u8) {
        self.growth_counter += bonus;
    }

    pub fn take_direction(&mut self, new_dir: Direction) {
        match (&self.direction, &new_dir) {
            // Prevent 180° turns
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => return,

            (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => {
                self.turning = Turning::Uptoleft;
                self.direction = new_dir
            }
            (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => {
                self.turning = Turning::Uptoright;
                self.direction = new_dir
            }
            (Direction::Down, Direction::Left) | (Direction::Right, Direction::Up) => {
                self.turning = Turning::Downtoleft;
                self.direction = new_dir
            }
            (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => {
                self.turning = Turning::Downtoright;
                self.direction = new_dir
            }
            _ => self.direction = new_dir,
        }
    }

    pub fn advance(&mut self, board_width: usize) -> Result<(), GameError> {
        if self.growth_counter == 0 {
            self.body.remove(0);
            self.body[0].1 = Cell::Tail;
        } else {
            self.growth_counter -= 1;
        }

        let new_head_index = self.body.len() - 1;

        // determine the shape of the body where the head just was
        self.body[new_head_index].1 = match self.turning {
            Turning::Uptoleft => Cell::Uptoleft,
            Turning::Uptoright => Cell::Uptoright,
            Turning::Downtoleft => Cell::Downtoleft,
            Turning::Downtoright => Cell::Downtoright,
            Turning::Keephorizontal => Cell::Horizontal,
            Turning::Keepvertical => Cell::Vertical,
        };

        let head = self.head_coordinate();

        let new_head = match self.direction {
            Direction::Left => match head.checked_sub(1) {
                Some(new_coordinate) => (new_coordinate, Cell::Headsleft),
                None => return Err(GameError::Collision),
            },
            Direction::Up => match head.checked_sub(board_width) {
                Some(new_coordinate) => (new_coordinate, Cell::Headsup),
                None => return Err(GameError::Collision),
            },
            Direction::Right => (head + 1, Cell::Headsright),
            Direction::Down => (head + board_width, Cell::Headsdown),
        };
        self.body.push(new_head);
        Ok(())
    }

    pub fn body_collides_with(&mut self, coordinate_to_check: usize) -> bool {
        // so as not to check the head
        for index in 0..self.body.len() - 2 {
            if self.body[index].0 == coordinate_to_check {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, PartialEq)]
pub enum Turning {
    Uptoleft,
    Uptoright,
    Downtoleft,
    Downtoright,
    Keephorizontal,
    Keepvertical,
}
