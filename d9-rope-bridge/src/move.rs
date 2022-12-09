use crate::direction::Direction;

#[derive(Debug)]
pub struct Move {
    pub direction: Direction,
    pub steps: usize,
}

impl Move {
    pub fn new(direction: Direction, steps: usize) -> Self {
        Move { direction, steps }
    }
}
