use std::collections::{HashMap, HashSet};

use crate::Valve;

pub fn best_move(
    minutes_left: i32,
    node: &Valve,
    valve_map: &HashMap<String, Valve>,
    opened: &mut HashSet<String>
) -> i32 {
    if minutes_left <= 0 {
        println!("No Minutes Left!");
        return 0;
    }

    println!("At {}", node.name);

    let mut max = 0;
    let open_current_valve = node.flow_rate != 0 && !opened.contains(&node.name);

    if open_current_valve {
        opened.insert(node.name.clone());
        println!("Opening {}", node.name);

        for next_valve_name in node.leads_to.iter() {
            let next_valve = &valve_map[next_valve_name];
            let mut r_max = best_move(minutes_left - 2, next_valve, valve_map, opened);
            r_max += node.flow_rate * (minutes_left - 1);

            max = max.max(r_max);
        }

        opened.remove(&node.name);
    } else {
        for next_valve_name in node.leads_to.iter() {
            let next_valve = &valve_map[next_valve_name];
            let r_max = best_move(minutes_left - 1, next_valve, valve_map, opened);
            max = max.max(r_max);
        }
    }

    max
}

pub fn solve_a(valve_map: HashMap<String, Valve>) -> i32 {

    valve_map.values().for_each(|v| println!("{v:?}"));

    let minutes_left = 30;

    let root_valve = Valve {
        flow_rate: 0,
        name: "root".to_string(),
        leads_to: valve_map.keys().cloned().collect(),
    };

    let mut opened = HashSet::new();

    best_move(minutes_left, &root_valve, &valve_map, &mut opened)
}
