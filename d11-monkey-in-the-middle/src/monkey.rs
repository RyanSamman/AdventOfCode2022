use std::{fmt::Display, mem};

use regex::Regex;

#[derive(Debug)]
pub struct Monkey {
    id: usize,
    operation: MonkeyOperation,
    items: Vec<usize>,
    test_number: usize,
    true_monkey_id: usize,
    false_monkey_id: usize,
    inspected_items: usize,
}

type MonkeyId = usize;

pub struct MonkeyThrow {
    monkey_id: MonkeyId,
    item: usize,
}

impl MonkeyThrow {
    fn new(monkey_id: MonkeyId, item: usize) -> Self {
        Self { monkey_id, item }
    }
}

impl Monkey {
    pub fn build(
        monkey_line: &str,
        starting_items_line: &str,
        operation_line: &str,
        test_line: &str,
        if_true_line: &str,
        if_false_line: &str,
    ) -> Option<Self> {
        let re_digits = Regex::new(r"\d+").expect("Valid Regex");

        let id = re_digits
            .captures(monkey_line)?
            .get(0)?
            .as_str()
            .parse()
            .ok()?;

        let test_number = re_digits
            .captures(test_line)?
            .get(0)?
            .as_str()
            .parse()
            .ok()?;

        let true_monkey_id = re_digits
            .captures(if_true_line)?
            .get(0)?
            .as_str()
            .parse()
            .ok()?;

        let false_monkey_id = re_digits
            .captures(if_false_line)?
            .get(0)?
            .as_str()
            .parse()
            .ok()?;

        let re_item = Regex::new(r".*: ").unwrap();
        let stripped_items = re_item.replace(starting_items_line, "");
        let items = stripped_items
            .split(", ")
            .map(|s| s.parse::<usize>().ok())
            .collect::<Option<Vec<usize>>>()?;

        let re_operation =
            Regex::new(r"(?P<arg1>old|\d+) (?P<op>\+|\*) (?P<arg2>old|\d+)").expect("Valid Regex");
        let caps = re_operation.captures(operation_line)?;

        let op = caps.name("op").unwrap().as_str();
        let arg1 = caps.name("arg1").unwrap().as_str();
        let arg2 = caps.name("arg2").unwrap().as_str();

        let operation = MonkeyOperation::build(op, arg1, arg2).expect("Valid MonkeyOperation");

        Some(Monkey {
            id,
            operation,
            items,
            test_number,
            true_monkey_id,
            false_monkey_id,
            inspected_items: 0,
        })
    }

    pub fn throw_items(&mut self, worry_level: usize) -> Vec<MonkeyThrow> {
        let items = mem::replace(&mut self.items, Vec::new());

        self.inspected_items += items.len();

        items
            .iter()
            .map(|old| self.operation.compute(*old))
            .map(|n| n / worry_level)
            .map(|new| MonkeyThrow::new(self.find_suitable_monkey_id(new), new))
            .collect()
    }

    fn find_suitable_monkey_id(&self, item: usize) -> MonkeyId {
        if item % self.test_number == 0 {
            self.true_monkey_id
        } else {
            self.false_monkey_id
        }
    }

    fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }
}

pub struct MonkeyManager {
    monkeys: Vec<Monkey>,
    worry_level: usize,
    common_divisor: usize,
}

impl MonkeyManager {
    pub fn new(worry_divisor: usize) -> Self {
        MonkeyManager {
            monkeys: Vec::new(),
            worry_level: worry_divisor,
            common_divisor: 1,
        }
    }

    pub fn add_monkey(&mut self, monkey: Monkey) {
        self.common_divisor *= monkey.test_number;
        self.monkeys.push(monkey)
    }

    pub fn simulate_round(&mut self) {
        // NOTE: Iterator syntax gets a mutable borrow to self.monkeys for the entire block,
        // so you can't change them later with the throws.
        // Getting an index allows a shorter lived borrow for the current monkey.
        for i in 0..self.monkeys.len() {
            let current_monkey = &mut self.monkeys[i];
            let throws = current_monkey.throw_items(self.worry_level);

            for MonkeyThrow { monkey_id, item } in throws {
                let managed_item = item % self.common_divisor;
                self.monkeys[monkey_id].add_item(managed_item);
            }
        }
    }

    pub fn monkey_inspections(&self) -> Vec<(MonkeyId, usize)> {
        self.monkeys
            .iter()
            .enumerate()
            .map(|(i, m)| (i, m.inspected_items))
            .collect()
    }

    pub fn monkey_business(&self) -> usize {
        let mut insp = self.monkey_inspections();

        insp.sort_by(|(_, a), (_, b)| b.cmp(a));

        println!("{insp:?}");

        insp[0].1 * insp[1].1
    }
}

impl Display for MonkeyManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String = String::new();

        for monkey in &self.monkeys[..] {
            let id = monkey.id;
            let items = monkey
                .items
                .iter()
                .map(usize::to_string)
                .intersperse(", ".to_string())
                .collect::<String>();
            s.push_str(format!("Monkey {id}: {items}\n").as_str());
        }

        s.fmt(f)
    }
}

#[derive(Debug)]
pub struct MonkeyOperation(MonkeyOperator, MonkeyOperand, MonkeyOperand);

impl MonkeyOperation {
    fn build(op: &str, arg1: &str, arg2: &str) -> Option<Self> {
        Some(MonkeyOperation(
            Self::parse_operator(op)?,
            Self::parse_operand(arg1)?,
            Self::parse_operand(arg2)?,
        ))
    }

    fn parse_operand(arg: &str) -> Option<MonkeyOperand> {
        use MonkeyOperand::*;
        match arg {
            "old" => Some(Old),
            s => Some(Literal(s.parse().ok()?)),
        }
    }

    fn parse_operator(op: &str) -> Option<MonkeyOperator> {
        use MonkeyOperator::*;
        match op {
            "*" => Some(Mult),
            "+" => Some(Add),
            _ => None,
        }
    }

    fn compute(&self, old_value: usize) -> usize {
        use MonkeyOperand::*;
        use MonkeyOperator::*;

        let arg1 = if let Literal(x) = self.1 {
            x
        } else {
            old_value
        };
        let arg2 = if let Literal(x) = self.2 {
            x
        } else {
            old_value
        };

        match self.0 {
            Add => arg1 + arg2,
            Mult => arg1 * arg2,
        }
    }
}

#[derive(Debug)]
pub enum MonkeyOperand {
    Old,
    Literal(usize),
}

#[derive(Debug)]
pub enum MonkeyOperator {
    Add,
    Mult,
}
