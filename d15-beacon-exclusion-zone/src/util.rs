use crate::beacon::Pos;

pub fn tuning_frequency((x, y): Pos) -> i64 {
    (x as i64) * 4000000 + (y as i64)
}

pub fn valid_pos(range: i32, (x, y): Pos) -> bool {
    0 <= x && x <= range && 0 <= y && y <= range
}

pub fn manhattan_distance((x1, y1): Pos, (x2, y2): Pos) -> i32 {
    (x2 - x1).abs() + (y2 - y1).abs()
}
