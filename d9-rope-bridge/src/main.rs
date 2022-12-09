use std::fs::read_to_string;

use direction::Direction;
use r#move::Move;

use crate::rope_simulation::RopeSimulation;

mod rope_simulation;
mod rope;
mod direction;
mod position;
mod r#move;
mod history;

fn parse_moves(filename: &str) -> Vec<Move> {
    let input = read_to_string(filename).unwrap();
    let mut moves = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let mv: Move = match line.split_whitespace().collect::<Vec<&str>>()[..] {
            [s, n] if n.parse::<usize>().is_ok() && ["R", "L", "U", "D"].contains(&s) => match s {
                "R" => Move::new(Direction::Right, n.parse::<usize>().unwrap()),
                "L" => Move::new(Direction::Left, n.parse::<usize>().unwrap()),
                "U" => Move::new(Direction::Up, n.parse::<usize>().unwrap()),
                "D" => Move::new(Direction::Down, n.parse::<usize>().unwrap()),
                _ => unreachable!(),
            },
            _ => panic!("Invalid Line {i}: {line}"),
        };

        moves.push(mv);
    }

    moves
}

fn main() {
    let moves = parse_moves("input.txt");

    let mut sim = RopeSimulation::new(10);

    moves.iter().for_each(|mv| sim.apply_move(mv));

    let revisited_count = sim.count_revisited_positions();

    println!("{revisited_count}");
}
