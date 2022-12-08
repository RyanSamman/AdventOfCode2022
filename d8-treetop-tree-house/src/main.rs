use std::fs::read_to_string;
mod scenic;

use crate::scenic::*;

fn parse_tree_grid(filename: &str) -> TreeGrid<i32> {
    let input = read_to_string(filename).unwrap();
    let lines: Vec<&str> = input.lines().collect();
    let size = lines.len();
    let mut tree_grid = create_empty_grid(size, 0);

    for (i, line) in lines.iter().enumerate() {
        let nums = line.chars();

        for (j, num) in nums.enumerate() {
            if !num.is_ascii_digit() {
                panic!("Invalid input on line {j}: {line}")
            }

            tree_grid[i][j] = num.to_digit(10 as u32).unwrap() as i32;
        }
    }

    tree_grid
}

fn main() {
    let tree_grid = parse_tree_grid("input.txt");
    let lm = left_map(&tree_grid);
    let rm = right_map(&tree_grid);
    let tm = top_map(&tree_grid);
    let bm = bottom_map(&tree_grid);

    let mm = mult_maps(&[&lm, &rm, &tm, &bm]);

    print_grid(&tree_grid);
    println!();
    print_grid(&tm);

    let c = find_most_scenic_value(&mm);
    println!("Most Scenic Value: {c}");

    /*
    let mm = min_maps(&[&lm, &rm, &tm, &bm]);

    print_grid(&tree_grid);
    println!();
    let vm = apply_visibility_map(&tree_grid, &mm);
    print_grid(&vm);

    let c = vm.iter().flatten().filter(|x| **x).count();
    println!("Visible Trees: {c}")
     */
}
