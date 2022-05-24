use std::collections::VecDeque;

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
    width: usize,
    height: usize,
    snake: VecDeque<Position>,
    direction: Direction,
    food_position: Position,
}

impl CentipedeGame {
    pub fn new(width: usize, height: usize) -> CentipedeGame {
        return CentipedeGame {
            width,
            height,
            snake: [((width - 2).max(0), height / 2)].into_iter().collect(),
            direction: Direction::Left,
            food_position: (2.min(width - 1), height / 2),
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

    pub fn loop_tick(&mut self) {}
}

#[cfg(test)]
mod tests {
    use crate::CentipedeGame;

    #[test]
    fn test() {
        println!("{:?}", CentipedeGame::new(10, 10));
    }
}
