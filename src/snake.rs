use crate::cell::Cell;
use crate::game::Direction;

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

            // so boilerplate...
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

    pub fn advance(&mut self, board_width: usize) {
        if self.growth_counter == 0 {
            self.body.remove(0);
        } else {
            self.growth_counter -= 1;
        }

        let new_head_index = self.body.len() - 1;

        // determine the shape of the body where the head was
        self.body[new_head_index].1 = match self.turning {
            Turning::Uptoleft => Cell::Uptoleft,
            Turning::Uptoright => Cell::Uptoright,
            Turning::Downtoleft => Cell::Downtoleft,
            Turning::Downtoright => Cell::Downtoright,
            Turning::Keephorizontal => Cell::Horizontal,
            Turning::Keepvertical => Cell::Vertical,
        };

        let new_head = match self.direction {
            Direction::Left => (self.head_coordinate() - 1, Cell::Headsleft),
            Direction::Right => (self.head_coordinate() + 1, Cell::Headsright),
            Direction::Up => (self.head_coordinate() - board_width, Cell::Headsup),
            Direction::Down => (self.head_coordinate() + board_width, Cell::Headsdown),
        };
        self.body.push(new_head);
    }

    pub fn body_collides_with(&mut self, coordinate_to_check: usize) -> bool {
        for index in 0..self.body.len() - 2 {
            // so as not to check the head
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
