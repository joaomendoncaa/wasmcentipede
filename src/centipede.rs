use std::collections::VecDeque;

use crate::random::random_range;

pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct CentipedeGame {
    pub width: usize,
    pub height: usize,
    pub centipede: VecDeque<Position>,
    pub direction: Direction,
    pub insect_position: Position,
    pub game_over: bool,
}

impl CentipedeGame {
    pub fn new(width: usize, height: usize) -> CentipedeGame {
        return CentipedeGame {
            width,
            height,
            centipede: [((width - 2).max(0), height / 2)].into_iter().collect(),
            direction: Direction::Left,
            insect_position: (2.min(width - 1), height / 2),
            game_over: false,
        };
    }

    pub fn update_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Up, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Left)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Right, Direction::Right) => {}
            (_, direction) => self.direction = direction,
        }
    }

    pub fn is_valid_position(&self, (x, y): Position) -> bool {
        x < self.width && y < self.height
    }

    pub fn spawn_insect(&mut self) {
        let free_positions = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|pos| !self.centipede.contains(pos))
            .collect::<Vec<Position>>();

        if free_positions.is_empty() {
            self.game_over = true;
            return;
        }

        self.insect_position = free_positions[random_range(0, free_positions.len() - 1, false)];
    }

    pub fn tick(&mut self) {
        if self.game_over && self.centipede.len() == 0 {
            return;
        }

        let (x, y) = self.centipede[0];
        let new_head = match &self.direction {
            Direction::Up => (x, (y - 1).max(1)),
            Direction::Down => (x, (y + 1).max(self.height)),
            Direction::Left => (x - 1.max(1), y),
            Direction::Right => (x + 1.max(self.width), y),
        };

        let is_out_of_bounds = !self.is_valid_position(new_head);
        let is_hitting_body = self.centipede.contains(&new_head);

        if is_out_of_bounds || is_hitting_body {
            self.game_over = true;
            return;
        }

        self.centipede.push_front(new_head);

        if new_head != self.insect_position {
            self.centipede.pop_back();
            return;
        }

        self.spawn_insect();
    }
}
