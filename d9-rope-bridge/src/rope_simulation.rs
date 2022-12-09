use crate::{history::History, position::Position, r#move::Move, rope::Rope};

pub struct RopeSimulation {
    rope: Rope,
    history: History,
}

impl RopeSimulation {
    pub fn new(size: usize) -> Self {
        RopeSimulation {
            rope: Rope::new(size),
            history: History::new(),
        }
    }

    pub fn apply_move(&mut self, Move { direction, steps }: &Move) {
        let diff_pos: Position = direction.into();

        println!("{direction:?}: {steps}");

        for _ in 0..*steps {
            self.rope.update_position(&diff_pos);
            self.history.add_history(self.rope.get_tail_position());
            // self.blit_state();
        }
        self.blit_state();
    }

    pub fn count_revisited_positions(&self) -> usize {
        self.blit_history();
        self.history.count_visited_positions()
    }

    fn blit_state(&self) {
        let debug = false;
        if !debug {
            return;
        }

        println!("{:?}", self.rope.segments);

        for i in (-5..=15).rev() {
            for j in -11..=11 {
                let mut found = false;

                for (k, pos) in self.rope.segments.iter().enumerate() {
                    if pos.into_tuple() == (i as i64, j) {
                        if k == 0 {
                            print!("H");
                        } else if k == self.rope.segments.len() - 1 {
                            print!("t");
                        } else {
                            print!("{k}");
                        }
                        found = true;
                        break;
                    }
                }

                if !found {
                    if (i, j) == (0, 0) {
                        print!("s");
                    } else {
                        print!(".");
                    }
                }
            }
            println!();
        }
        println!();
    }

    fn blit_history(&self) {
        let debug = true;
        if !debug {
            return;
        }

        println!("{:?}", self.rope.segments);

        for i in (-5..=15).rev() {
            for j in -11..=11 {
                if (i, j) == (0, 0) {
                    print!("s");
                } else if self.history.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
    }
}
