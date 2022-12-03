#![feature(trait_alias)]
#![feature(result_option_inspect)]
#![feature(exact_size_is_empty)]

mod rucksack;
use std::env;

use crate::rucksack::{group_rucksacks_into_threes, read_file_rucksacks, Rucksack, RucksackGroup};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let rucksacks = read_file_rucksacks("src/contents.txt").unwrap();
    let priority: i32 = rucksacks
        .iter()
        // .inspect(|rs| println!("{:?}", rs))
        // .inspect(|rs| println!("{:?}", rs.compute_rucksack_common_items().iter().map(|c| (c.clone() as char)).collect::<Vec<char>>()))
        .map(Rucksack::compute_priority)
        // .inspect(|rs| println!("{:?}", rs))
        .sum();

    println!("Sum of priorities: {}", priority);

    let group_priorities: i32 = group_rucksacks_into_threes(rucksacks)
        .iter()
        .map(RucksackGroup::compute_priority)
        .sum();
    println!("Sum of Group Priorities: {}", group_priorities);
}
