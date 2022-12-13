#![feature(iter_intersperse)]

use monkey::{Monkey, MonkeyManager};
use std::fs::read_to_string;
mod monkey;

fn read_monkeys(file: &str, monkeys: &mut MonkeyManager) {
    let input = read_to_string(file).unwrap();
    let mut lines = input.lines();

    while let Some(mut line) = lines.next() {
        if line.is_empty() {
            line = lines.next().unwrap();
        }

        let monkey_line = line;

        let starting_items_line = lines.next().unwrap();
        let operation_line = lines.next().unwrap();
        let test_line = lines.next().unwrap();
        let if_true_line = lines.next().unwrap();
        let if_false_line = lines.next().unwrap();

        let monkey = Monkey::build(
            monkey_line,
            starting_items_line,
            operation_line,
            test_line,
            if_true_line,
            if_false_line,
        )
        .unwrap();

        println!("{monkey:#?}");
        monkeys.add_monkey(monkey);
    }
}

fn main() {
    let mut monkeys = MonkeyManager::new(1);
    read_monkeys("input.txt", &mut monkeys);

    for _ in 1..=10000 {
        monkeys.simulate_round();
    }

    println!("{}", monkeys.monkey_business())
}
