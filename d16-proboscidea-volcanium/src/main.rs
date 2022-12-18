use std::collections::HashMap;

mod solution_a;
mod solution_b;

use itertools::Itertools;
use regex::Regex;

type ValveID = usize;

use crate::{solution_a::solve_a, solution_b::solve_b};

#[derive(Debug)]
pub struct Valve {
    pub id: usize,
    pub name: String,
    pub flow_rate: i32,
    pub leads_to: Vec<ValveID>,
}

fn parse_valves(input: &str) -> Vec<Valve> {
    pub struct ParsedValve {
        pub name: String,
        pub flow_rate: i32,
        pub leads_to: Vec<String>,
    }

    fn parse_valve(line: &str) -> ParsedValve {
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

        ParsedValve {
            name,
            flow_rate,
            leads_to,
        }
    }

    fn resolve_valves(vs: Vec<ParsedValve>) -> Vec<Valve> {
        let name_to_id_resolver: HashMap<String, ValveID> =
            vs.iter().enumerate().map(|(i, v)| (v.name.clone(), i)).collect();

        let mut valves = Vec::new();

        for pv in vs {
            let v = Valve {
                id: name_to_id_resolver[&pv.name],
                name: pv.name,
                flow_rate: pv.flow_rate,
                leads_to: pv.leads_to.iter().map(|vn| name_to_id_resolver[vn]).collect_vec(),
            };

            valves.push(v);
        }

        valves
    }

    let parsed_valves = input
        .lines()
        .map(parse_valve)
        .collect_vec();

    resolve_valves(parsed_valves)
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
