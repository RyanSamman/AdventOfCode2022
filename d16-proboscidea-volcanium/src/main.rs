use std::collections::HashMap;

mod solution_a;
mod solution_b;

use regex::Regex;

use crate::{solution_b::solve_b, solution_a::solve_a};

#[derive(Debug)]
pub struct Valve {
    pub name: String,
    pub flow_rate: i32,
    pub leads_to: Vec<String>,
}

fn parse_valves(input: &str) -> HashMap<String, Valve> {
    fn parse_valve(line: &str) -> Valve {
        let re = Regex::new(
            r"Valve (?P<name>[A-Z]{2}) has flow rate=(?P<flow_rate>\d+); tunnel(s)? lead(s)? to valve(s)? (?P<leads_to>([A-Z]{2}, )*[A-Z]{2})"
        ).unwrap();

        let caps = re.captures(line).unwrap();

        let name = caps["name"].to_string();
        let flow_rate = caps["flow_rate"].parse().unwrap();
        let leads_to = caps["leads_to"]
            .split(", ")
            .map(ToString::to_string)
            .collect();

        Valve {
            name,
            flow_rate,
            leads_to,
        }
    }

    input
        .lines()
        .map(parse_valve)
        .map(|v| (v.name.clone(), v))
        .collect()
}

fn main() {
    enum Part {
        A,
        B,
    }

    let part = Part::A;
    let debug = true;
    let debug_input = include_str!("./test-input.txt");
    let input = include_str!("./input.txt");

    match (part, debug) {
        (Part::A, true) => {
            let valve_map = parse_valves(debug_input);
            let result = solve_a(valve_map);
            assert_eq!(result, 1);
        }
        (Part::A, false) => {
            let valve_map = parse_valves(input);
            let result = solve_a(valve_map);
            assert_eq!(result, 1);
        }
        (Part::B, true) => {
            let valve_map = parse_valves(debug_input);
            let result = solve_b(valve_map);
            assert_eq!(result, 1);
        }
        (Part::B, false) => {
            let valve_map = parse_valves(input);
            let result = solve_b(valve_map);
            assert_eq!(result, 1);
        }
    }
}
