use std::{
    collections::BTreeSet,
    fs::File,
    io::{BufRead, BufReader, Error, Result},
    usize,
};

#[derive(Debug, Clone)]
pub struct Rucksack {
    c1: String,
    c2: String,
    all: String,
}

impl Rucksack {
    pub fn new(s: String) -> Self {
        let (c1, c2) = s.split_at(s.len() / 2);
        Rucksack {
            c1: c1.to_string(),
            c2: c2.to_string(),
            all: s,
        }
    }
    pub fn compute_rucksack_common_items(&self) -> BTreeSet<u8> {
        let compartment1_set = BTreeSet::from_iter(self.c1.clone().into_bytes().into_iter());
        let compartment2_set = BTreeSet::from_iter(self.c2.clone().into_bytes().into_iter());

        compartment1_set
            .intersection(&compartment2_set)
            .cloned()
            .collect()
    }

    // NOTE: Assumes item is a valid character [a-zA-Z]
    fn compute_item_priority(item: u8) -> i32 {
        match item as i32 {
            i @ 65..=90 => i - 65 + 27,
            i @ 97..=122 => i - 97 + 1,
            _ => panic!("Invalid Rucksack compartment Item."),
        }
    }

    pub fn compute_priority(&self) -> i32 {
        self.compute_rucksack_common_items()
            .into_iter()
            .map(Rucksack::compute_item_priority)
            .sum()
    }
}

#[derive(Debug, Clone)]
pub struct RucksackGroup(pub Rucksack, pub Rucksack, pub Rucksack);

impl RucksackGroup {
    pub fn new(rucksacks: &[Rucksack]) -> Self {
        assert_eq!(rucksacks.len(), 3);
        RucksackGroup(
            rucksacks[0].clone(),
            rucksacks[1].clone(),
            rucksacks[2].clone(),
        )
    }

    pub fn find_common_item(&self) -> u8 {
        let set0 = BTreeSet::from_iter(self.0.clone().all.into_bytes().into_iter());
        let set1 = BTreeSet::from_iter(self.1.clone().all.into_bytes().into_iter());
        let set2 = BTreeSet::from_iter(self.2.clone().all.into_bytes().into_iter());

        set0.intersection(&set1)
            .cloned()
            .collect::<BTreeSet<u8>>()
            .intersection(&set2)
            .cloned()
            .collect::<BTreeSet<u8>>()
            .first()
            .unwrap()
            .clone()
    }

    pub fn compute_priority(&self) -> i32 {
        Rucksack::compute_item_priority(self.find_common_item())
    }
}

struct RucksackParserItem {
    line_number: usize,
    rucksack_string: String,
}

impl RucksackParserItem {
    fn parse_rucksack_string(&self) -> Rucksack {
        let s = self.rucksack_string.clone();
        return Rucksack::new(s);
    }

    fn is_valid_rucksack_string(&self) -> bool {
        self.rucksack_string.chars().all(|b| (b >= 'A' && b <= 'z'))
    }

    fn generate_parse_error(&self) -> Error {
        let errstr = format!(
            "Error Parsing line {}:\n
{} contains values outside of [a-zA-Z]",
            self.line_number, self.rucksack_string
        );

        Error::new(std::io::ErrorKind::InvalidInput, errstr)
    }
}

pub trait Iter = Iterator<Item = Result<String>>;

pub struct RucksackParser<I: Iter> {
    line_number: usize,
    source: I,
}

impl<I: Iter> RucksackParser<I> {
    pub fn new(source: I) -> RucksackParser<I> {
        return RucksackParser {
            line_number: 0,
            source,
        };
    }
}

impl<I: Iter> Iterator for RucksackParser<I> {
    type Item = Result<Rucksack>;

    fn next(&mut self) -> Option<Result<Rucksack>> {
        let x = match self.source.next()? {
            Err(err) => Err(err),
            Ok(rucksack_string) => {
                self.line_number += 1;

                let parse_item = RucksackParserItem {
                    line_number: self.line_number,
                    rucksack_string,
                };

                if parse_item.is_valid_rucksack_string() {
                    Ok(parse_item.parse_rucksack_string())
                } else {
                    Err(parse_item.generate_parse_error())
                }
            }
        };

        Some(x)
    }
}

pub fn read_file_rucksacks(filename: &str) -> Result<Vec<Rucksack>> {
    let rucksacks = RucksackParser::new(BufReader::new(File::open(filename)?).lines())
        .collect::<Result<Vec<Rucksack>>>()?;

    if rucksacks.len() % 3 != 0 {
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "The number of rucksacks are not divisible by 3.",
        ))
    } else {
        Ok(rucksacks)
    }
}

pub fn group_rucksacks_into_threes(rucksacks: Vec<Rucksack>) -> Vec<RucksackGroup> {
    // assert it's divisibility by 3
    assert!(rucksacks.len() % 3 == 0);
    rucksacks.chunks_exact(3).map(RucksackGroup::new).collect()
}
