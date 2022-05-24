use std::collections::HashSet;

pub type Position = (usize, usize);
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct CentipedeGame {
    width: usize,
    height: usize,
    snake: Vec<Position>,
    direction: Direction,
    food_position: Position,
}

impl CentipedeGame {
    pub fn new(width: usize, height: usize) -> CentipedeGame {
        return CentipedeGame {
            width,
            height,
            snake: vec![((width - 2).max(0), height / 2)],
            direction: Direction::Left,
            food_position: (2.min(width - 1), height / 2),
        };
    }
}
