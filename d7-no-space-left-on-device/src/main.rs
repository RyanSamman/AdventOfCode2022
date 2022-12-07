use std::fs::read_to_string;

mod directory_manager;
use directory_manager::DirectoryManager;

fn is_numeric(s: &str) -> bool {
    s.chars().all(|x| x.is_ascii_digit())
}

fn parse_directory(filename: &str) -> DirectoryManager {
    let s = read_to_string(filename).unwrap();
    let mut lines = s.lines().enumerate();

    let mut manager = DirectoryManager::new();

    while let Some((i, line)) = lines.next() {
        let tokens = line.split(" ").collect::<Vec<&str>>();
        // println!("{manager:?}");
        // println!("{tokens:?}");
        match &tokens[..] {
            ["$", "cd", dir] => manager.cd(dir),
            ["$", "ls"] => {}
            ["dir", dirname] => manager.create_dir(dirname),
            [size, filename] if is_numeric(size) => {
                let fsize = size.parse::<u64>().unwrap();
                manager.create_file(filename, fsize);
            }
            _ => panic!("Invalid Input on line {i}: {line}"),
        };
    }

    manager
}

fn main() {
    let tdir = parse_directory("input.txt");
    //println!("{tdir:#?}");
    let sizes = tdir.compute_dir_sizes();
    println!("Sizes: {sizes:?}");

    let total_size = sizes[0];
    let mut deletion_candidates = sizes
        .iter()
        .map(|x| (*x, total_size - *x))
        .filter(|(_, x)| *x <= (70000000 - 30000000))
        .collect::<Vec<(u64, u64)>>();

    deletion_candidates.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Candidates for Deletion: {:?}", deletion_candidates);

    println!(
        "Smallest Candidate: {:?}",
        deletion_candidates.first().unwrap().0
    );
}
