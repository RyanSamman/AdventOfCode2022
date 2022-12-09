use std::collections::HashMap;

use crate::position::Position;

#[derive(Debug)]
pub struct History {
    map: HashMap<(i64, i64), usize>,
}

impl History {
    pub fn new() -> Self {
        History {
            map: HashMap::new(),
        }
    }

    pub fn add_history(&mut self, pos: &Position) {
        *self.map.entry(pos.into_tuple()).or_insert(0) += 1;
    }

    pub fn contains(&self, pos: &(i64, i64)) -> bool {
        self.map.contains_key(pos)
    }

    pub fn count_visited_positions(&self) -> usize {
        self.map.values().count()
    }

}
