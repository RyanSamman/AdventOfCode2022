use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use priority_queue::DoublePriorityQueue;

use crate::{Valve, ValveID};

// Time it takes to move from one valve to the next.
// Moving from one valve to the another adjacent valve takes 1 minute.
fn compute_weight(
    from: ValveID,
    to: ValveID,
    valves: &Vec<Valve>,
    computed_weights: &HashMap<(ValveID, ValveID), i32>,
) -> i32 {
    let from_valve = &valves[from];

    let mut visited: HashSet<ValveID> = HashSet::new();
    visited.insert(from);

    // 'Could' use a linked list,
    // but would need to profile it to see whether it'd actually be faster than a vec
    let mut queue: Vec<(ValveID, i32)> = Vec::new();
    queue.extend(from_valve.leads_to.iter().map(|&id| (id, 1)));

    while !queue.is_empty() {
        let (current_valve_id, distance) = queue.remove(0);

        if current_valve_id == to {
            return distance;
        }
        // NOTE: Can take advantage of the fact that we've
        // computed distances between other nodes before - look at compute_dequeue_weight
        /*
         *
         else if computed_weights.contains_key(&(current_valve_id, to)) {
            // This doesn't work because it gives _a_ distance, but not the shortest
            return computed_weights[&(current_valve_id, to)] + 1;
        }
         */

        let current_valve = &valves[current_valve_id];

        visited.insert(current_valve_id);
        queue.extend(current_valve.leads_to.iter().map(|&id| (id, distance + 1)));
    }

    panic!("Cannot reach a node!");
}

fn compute_dequeue_weight(
    from: ValveID,
    to: ValveID,
    valves: &Vec<Valve>,
    computed_weights: &HashMap<(ValveID, ValveID), i32>,
) -> i32 {
    let from_valve = &valves[from];

    let mut visited: HashSet<ValveID> = HashSet::new();
    visited.insert(from);

    let mut queue: DoublePriorityQueue<ValveID, i32> = DoublePriorityQueue::new();
    queue.extend(from_valve.leads_to.iter().map(|&id| (id, 1)));

    while !queue.is_empty() {
        let (current_valve_id, distance) = queue.pop_min().unwrap();

        if current_valve_id == to {
            return distance;
        }
         else if computed_weights.contains_key(&(current_valve_id, to)) {
            let distance_to_end = computed_weights[&(current_valve_id, to)] + 1;
            queue.push_decrease(to, distance_to_end + distance - 1);
        } else {
            let current_valve = &valves[current_valve_id];
            visited.insert(current_valve_id);
            for &id in current_valve.leads_to.iter() {
                queue.push_decrease(id, distance + 1);
            }
        }
    }

    panic!("Cannot reach a node!");
}

// NOTE: Refactored to use a number to index a vec rather than a string into a hashmap
// to avoid cloning the reference over and over
fn compute_weights(valves: &Vec<Valve>) -> HashMap<(ValveID, ValveID), i32> {
    let perms = valves
        .iter()
        .filter(|v| v.flow_rate != 0)
        .map(|v| v.id)
        .permutations(2)
        .map(|mut perm| (perm.remove(0), perm.remove(0)));

    let mut weights = HashMap::new();

    for (v1, v2) in perms {
        let weight = compute_dequeue_weight(v1, v2, valves, &weights);
        weights.insert((v1, v2), weight);
    }

    weights
}

// Maximum flow from next actions
// Assume already opened current_valve and is moving onto the next
fn max_flow(
    minutes_left: i32,
    current_valve: &Valve,
    valves: &Vec<Valve>,
    computed_weights: &HashMap<(ValveID, ValveID), i32>,
    visited: &mut HashSet<ValveID>,
) -> i32 {
    if minutes_left <= 0 {
        return 0;
    }

    let mut max = 0;

    for next_valve in valves {
        if visited.contains(&next_valve.id) || next_valve.flow_rate == 0 {
            continue;
        }

        visited.insert(next_valve.id);

        // Hope this little maneuver doesn't cost us 51 years!
        let little_maneuver_cost = computed_weights[&(current_valve.id, next_valve.id)];
        // Opening the current valve cost 1 minute, and moving to the next valve gives us
        let m = minutes_left - little_maneuver_cost - 1;

        let mut flow = next_valve.flow_rate * m;
        flow += max_flow(m, next_valve, valves, computed_weights, visited);
        max = max.max(flow);

        visited.remove(&next_valve.id);
    }

    max
}

pub fn solve_a(valves: Vec<Valve>) -> i32 {
    valves.iter().for_each(|v| println!("{v:?}"));
    let minutes_left = 30;

    let root_valve = Valve {
        id: valves.len(), // will panic if attempting to check index
        name: "root".to_string(),
        flow_rate: 0,
        leads_to: (0..valves.len()).into_iter().collect_vec(),
    };

    let mut weights = compute_weights(&valves);

    weights.iter()
           .sorted()
           .for_each(|(&(f, t), d)| println!("{} - {}: {d}", valves[f].name, valves[t].name));

    // // Moving from the root to any other node costs 1
    for &v_id in root_valve.leads_to.iter() {
        weights.insert((root_valve.id, v_id), 1);
    }

    let mut visited = HashSet::new();

    max_flow(minutes_left, &root_valve, &valves, &weights, &mut visited)
}
