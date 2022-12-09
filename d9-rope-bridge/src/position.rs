use std::ops::{Add, Sub};

use crate::direction::Direction;

#[derive(Debug, Clone)]
pub struct Position {
    pub row: i64,
    pub col: i64,
}

impl Position {
    pub fn new(x: i64, y: i64) -> Self {
        Self { row: x, col: y }
    }

    fn op(p1: &Position, p2: &Position, f: fn(i64, i64) -> i64) -> Position {
        Position::new(f(p1.row, p2.row), f(p1.col, p2.col))
    }

    pub fn add(&self, p: &Position) -> Position {
        Position::op(&self, p, Add::add)
    }

    pub fn sub(&self, p: &Position) -> Position {
        Position::op(&self, p, Sub::sub)
    }

    pub fn into_tuple(&self) -> (i64, i64) {
        (self.row, self.col)
    }
}

impl From<(i64, i64)> for Position {
    fn from((x, y): (i64, i64)) -> Self {
        Position::new(x, y)
    }
}

impl From<Direction> for Position {
    fn from(value: Direction) -> Self {
        (&value).into()
    }
}

impl From<&Direction> for Position {
    fn from(direction: &Direction) -> Self {
        use Direction::*;

        match direction {
            Right => (0, 1).into(),
            Left => (0, -1).into(),
            Up => (1, 0).into(),
            Down => (-1, 0).into(),
        }
    }
}
