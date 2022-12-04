use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

use itertools::Itertools;
use regex::Regex;

type ElfRange = (u8, u8, u8, u8);

// s must be in the form xx-xx,xx-xx
fn parse_elf_range(s: &String) -> ElfRange {
    let r = Regex::new("[,-]").expect("Regex is Invalid");
    let elf_range: ElfRange = r
        .splitn(s.as_str(), 4)
        .map(|x| x.parse::<u8>().unwrap())
        .next_tuple()
        .unwrap();

    elf_range
}

/*
 * Checks whether the two ranges fully contain eachother, inclusive
 *
 * # Examples:
 *
 * |---|
 * |---|
 *
 * |---|
 *  |-|
 *
 *  |-|
 * |---|
 *
 * # Invariants
 * a1 <= a2
 * b1 <= b2
 */
fn is_contained((a1, a2, b1, b2): &ElfRange) -> bool {
    (a1 <= b1 && a2 >= b2) || (b1 <= a1 && b2 >= a2)
}

// easier to check if not overlapping?
fn overlap((a1, a2, b1, b2): &ElfRange) -> bool {
    !((a1 < b1 && a2 < b1) || (b1 < a1 && b2 < a1))
}

fn read_elf_range_lines(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let line_matcher = Regex::new(r"^\d{1,2}-\d{1,2},\d{1,2}-\d{1,2}$").expect("Regex is Invalid");

    let mut range_lines: Vec<String> = Vec::new();
    let mut i = 0;

    for line in lines {
        let s = line?;
        i += 1;

        if line_matcher.is_match(&s) {
            range_lines.push(s.clone());
        } else {
            return Err(Error::new(ErrorKind::Interrupted,
                                  format!("Invalid Elf Line {}: {}", i, s)));
        }
    }

    Ok(range_lines)
}

fn main() {

    let contained_count = read_elf_range_lines("src/input.txt")
        .unwrap()
        .iter()
        .map(parse_elf_range)
        .filter(overlap)
        // .filter(is_contained)
        // .inspect(|x| println!("{:?}", x))
        .count();

    println!("Number of contained Elves: {}", contained_count);
}
