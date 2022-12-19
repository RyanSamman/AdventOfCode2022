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
        } else if computed_weights.contains_key(&(current_valve_id, to)) {
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

pub fn solve_a(valves: Vec<Valve>) -> i32 {
    valves.iter().for_each(|v| println!("{v:?}"));
    let minutes_left = 30;

    let mut weights = compute_weights(&valves);

    let flow_valves = valves
        .iter()
        .enumerate()
        .filter(|(i, v)| v.flow_rate != 0)
        .map(|(i, v)| i).collect_vec();


    let mut max = 0;

    let get_flow = |id: ValveID| valves[id].flow_rate;

    // Bug somewhere, but I can't find it ;/
    for mut path in flow_valves.iter().copied().permutations(flow_valves.len() - 1) {
        let mut previous_valve_id = path.pop().unwrap();
        let mut t = minutes_left - 2;
        let mut flow = t * get_flow(previous_valve_id);

        while !path.is_empty() {
            let current_valve_id = path.pop().unwrap();

            let time_between_vents = weights[&(previous_valve_id, current_valve_id)];

            t -= time_between_vents + 1;

            if t <= 0 {
                break;
            }

            flow += t * get_flow(current_valve_id);
            previous_valve_id = current_valve_id;
        }

        max = max.max(flow);
    }

    max
}
