#![feature(box_syntax)]

use std::{fmt::Display, fs::read_to_string};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct RawRearrangementProcedure {
    number_of_stacks: usize,
    crate_stack_lines: Vec<String>,
    move_lines: Vec<String>,
}

#[derive(Debug)]
enum ParseState {
    Crates,
    Numbers,
    Space,
    Moves,
    Unknown,
}

fn read_procedure(filename: &str) -> RawRearrangementProcedure {
    use ParseState::{Crates, Moves, Numbers, Space, Unknown};

    let data = read_to_string(filename).unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let line_matcher = Regex::new(r"^(    |\[\w\] )*(   |\[\w\])$").unwrap();
    let stack_number_regex = Regex::new(r"^ (\d   )*\d$").unwrap();
    let move_regex = Regex::new(r"^move \d+ from \d+ to \d+$").unwrap();

    let mut crate_stacks: Vec<String> = Vec::new();
    let mut number_of_stacks: usize = 0;
    let mut moves: Vec<String> = Vec::new();

    let mut i = 0;

    let mut parse_state: ParseState;
    for line in lines {
        let s = line.to_string();
        i += 1;

        if line_matcher.is_match(&s) {
            parse_state = Crates;
        } else if stack_number_regex.is_match(&s) {
            parse_state = Numbers;
        } else if s.is_empty() {
            parse_state = Space;
        } else if move_regex.is_match(&s) {
            parse_state = Moves;
        } else {
            parse_state = Unknown;
        }

        match parse_state {
            Crates => crate_stacks.push(s.clone()),
            Numbers => number_of_stacks = s.split(" ").last().unwrap().parse::<usize>().unwrap(),
            Space => continue,
            Moves => moves.push(s),
            Unknown => panic!("Line {i}: \"{s}\" - {parse_state:?}"),
        }
    }

    crate_stacks.reverse();

    RawRearrangementProcedure {
        crate_stack_lines: crate_stacks,
        number_of_stacks,
        move_lines: moves,
    }
}

#[derive(Debug)]
struct Move {
    count: i32,
    from: usize,
    to: usize,
}

impl From<&String> for Move {
    fn from(value: &String) -> Self {
        let r = Regex::new(r" ").unwrap();

        let (_, count, _, from, _, to) = r.splitn(value, 6).next_tuple().unwrap();

        Move {
            count: count.parse().unwrap(),
            from: from.parse::<usize>().unwrap() - 1,
            to: to.parse::<usize>().unwrap() - 1,
        }
    }
}

type StackCommand = (usize, char);

fn parse_crates(s: &String) -> Vec<StackCommand> {
    let mut v = Vec::new();
    let mut i = 0;
    let mut index = i + 1;

    let chars = s.as_bytes();

    while index < s.len() {
        let c = chars[index];

        if c.is_ascii_alphabetic() {
            v.push((i, c as char));
        }

        i += 1;
        index = 4 * i + 1;
    }

    v
}

struct EmptyStacks {
    stack_count: usize,
    stacks: Box<Vec<Box<Vec<char>>>>,
}

impl EmptyStacks {
    fn build_empty_stack(stack_count: usize) -> EmptyStacks {
        let mut s = EmptyStacks {
            stack_count,
            stacks: box Vec::new(),
        };

        for _ in 0..stack_count {
            s.stacks.push(box Vec::new());
        }

        s
    }
}

struct Stacks {
    stack_count: usize,
    stacks: Box<Vec<Box<Vec<char>>>>,
}

impl Stacks {
    fn build_stack(
        EmptyStacks {
            stack_count,
            mut stacks,
        }: EmptyStacks,
        crate_lines: Vec<Vec<StackCommand>>,
    ) -> Stacks {
        for crate_line in crate_lines {
            for (i, c) in crate_line {
                assert!(i < stack_count);
                stacks[i].push(c);
            }
        }

        Stacks {
            stack_count,
            stacks,
        }
    }


}

trait CrateMover {
    fn move_crates(s: &mut Stacks, mv: Move);
}

struct CrateMover9000;

impl CrateMover for CrateMover9000 {
    fn move_crates(s: &mut Stacks, Move { count, from, to }: Move) {
        for _ in 0..count {
            let x = s.stacks[from].pop().unwrap();
            s.stacks[to].push(x);
        }
    }
}

struct CrateMover9001;

impl CrateMover for CrateMover9001 {
    fn move_crates(s: &mut Stacks, Move { count, from, to }: Move) {

        let mut crate_list = Vec::new();
        for _ in 0..count {
            let x = s.stacks[from].pop().unwrap();
            crate_list.push(x);
        }

        crate_list.reverse();
        s.stacks[to].append(&mut crate_list);
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = "".to_string();

        if self.stack_count != 0 {
            for i in 0..self.stack_count {
                let c = self.stacks[i].last().unwrap();
                s.push(*c);
            }
        }

        s.fmt(f)
    }
}

fn main() {
    let RawRearrangementProcedure {
        number_of_stacks,
        crate_stack_lines,
        move_lines,
    } = read_procedure("src/input.txt");

    let es = EmptyStacks::build_empty_stack(number_of_stacks);

    let crates = crate_stack_lines
        .iter()
        .map(parse_crates)
        .collect::<Vec<Vec<StackCommand>>>();

    let mut s = Stacks::build_stack(es, crates);

    move_lines
        .iter()
        .map(Move::from)
        .for_each(|mv| CrateMover9001::move_crates(&mut s, mv));

    println!("{s}");
}
