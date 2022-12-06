use std::{collections::{HashSet, LinkedList}, fs::read_to_string, hash::Hash};

fn is_all_unique<T>(i: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash + Clone,
{
    let mut uniq = HashSet::new();
    i.into_iter().all(move |x| uniq.insert(x))
}

fn find_marker(s: String, size: usize) -> usize {
    let mut b = s.bytes();

    let mut i = size;


    let mut cs: LinkedList<u8> = LinkedList::new();

    for _ in 0..size {
        cs.push_back(b.next().unwrap());
    }

    while !is_all_unique(cs.clone()) {

        if let Some(c) = b.next() {
            i += 1;
            cs.push_back(c);
            cs.pop_front();
        } else {
            panic!("No marker!");
        }

    }

    i
}

fn main() {
    let filenames = [
        "test1-input.txt",
        "test2-input.txt",
        "test3-input.txt",
        "test4-input.txt",
        "test5-input.txt",
        "input.txt",
    ];

    filenames
        .iter()
        .map(|f| read_to_string(f).unwrap())
        .map(|s| find_marker(s, 14))
        .zip(filenames)
        .for_each(|(i, f)| println!("{f}: {i}"));
}
