use crate::direction::Direction;
use crate::position::Position;

pub struct Rope {
    pub segments: Vec<Position>,
}

impl Rope {
    pub fn new(rope_size: usize) -> Self {
        assert!(rope_size > 0);
        let mut segments = Vec::with_capacity(rope_size);

        for _ in 0..rope_size {
            segments.push(Position::new(0, 0));
        }

        Rope { segments }
    }

    fn keep_up_delta(Position { row, col }: &Position) -> Position {
        use Direction::*;

        if *row == 0 {
            if *col > 1 {
                return Right.into();
            } else if *col < -1 {
                return Left.into();
            } else {
                return (0, 0).into();
            }
        }

        if *col == 0 {
            if *row > 1 {
                return Up.into();
            } else if *row < -1 {
                return Down.into();
            } else {
                return (0, 0).into();
            }
        }

        let mut dk: Position = (0, 0).into();

        if *col >= 1 {
            dk = dk.add(&Right.into());
        }

        if *col <= -1 {
            dk = dk.add(&Left.into());
        }

        if *row >= 1 {
            dk = dk.add(&Up.into());
        }

        if *row <= -1 {
            dk = dk.add(&Down.into());
        }

        dk
    }

    pub fn update_position(&mut self, dhead: &Position) {
        // let mut prev_node = self.segments[0].clone();
        self.segments[0] = self.segments[0].add(&dhead);

        for i in 1..self.segments.len() {
            let ht_diff = self.segments[i - 1].sub(&self.segments[i]);

            if ht_diff.col > 1 || ht_diff.col < -1 || ht_diff.row > 1 || ht_diff.row < -1 {
                self.segments[i] = self.segments[i].add(&Rope::keep_up_delta(&ht_diff));
                // swap(&mut self.segments[i], &mut prev_node);
            }
        }
    }

    /*
     *     fn update_position(&mut self, dhead: &Position) {
        let prev_head = self.head.clone();
        self.head = self.head.add(&dhead);

        let ht_diff = self.head.sub(&self.tail);

        if ht_diff.y > 1 || ht_diff.y < -1 || ht_diff.x > 1 || ht_diff.x < -1 {
            self.tail = prev_head;
        }
    }
     */

    pub fn get_tail_position(&self) -> &Position {
        &self.segments.last().unwrap()
    }
}
