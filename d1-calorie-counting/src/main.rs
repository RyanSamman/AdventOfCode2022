use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[test]
fn test_read_file() {
    let expected_elf_calories = vec![6000, 4000, 11000, 24000, 10000];

    let actual_elf_calories = read_file("src/test-elf-calories.txt").unwrap();

    assert_eq!(expected_elf_calories, actual_elf_calories);
}

fn read_file(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut elf_calories: Vec<i32> = Vec::new();
    let mut current_elf_calories = 0;

    for line_or_error in reader.lines() {
        let line = line_or_error?;
        if line.is_empty() && current_elf_calories != 0 {
            elf_calories.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            let calories = line.parse::<i32>()?;
            current_elf_calories += calories;
        }
    }

    if current_elf_calories != 0 {
        elf_calories.push(current_elf_calories)
    }

    Ok(elf_calories)
}

fn print_elves_calories(elves: &[i32]) {
    elves.iter().for_each(|i| println!("{}", i));
}

fn main() {
    match read_file("elf-calories.txt") {
        Ok(mut elf_calories) => {
            elf_calories.sort_by(|a, b| b.cmp(a));

            println!("All Elf Calories:");
            print_elves_calories(&elf_calories);

            println!("Number of Elves: {}", elf_calories.len());

            if !elf_calories.is_empty() {
                println!("Greatest Elf Calories: {}", elf_calories[0]);
            }


            if elf_calories.len() >= 3 {
                println!("Greatest three Elf Calories: {}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
            }

        }
        Err(err) => println!("{}", err),
    }
}
