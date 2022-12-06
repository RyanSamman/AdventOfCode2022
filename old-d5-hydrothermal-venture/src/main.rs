use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Result},
};

use itertools::Itertools;
use regex::Regex;
use split_iter::Splittable;

fn read_vent_lines(filename: &str) -> Result<Vec<String>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    // NOTE: matches xxx-xxx -> xxx-xxx
    // where xxx is a number between 0 and 999
    let line_matcher =
        Regex::new(r"^\d{1,3},\d{1,3} -> \d{1,3},\d{1,3}$").expect("Regex is Invalid");

    let mut range_lines: Vec<String> = Vec::new();
    let mut i = 0;

    for line in lines {
        let s = line?;
        i += 1;

        if line_matcher.is_match(&s) {
            range_lines.push(s.clone());
        } else {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid vent Line {}: {}", i, s),
            ));
        }
    }

    Ok(range_lines)
}

type Quaduple<T> = (T, T, T, T);

fn parse_vent_line(s: &String) -> Quaduple<i32> {
    let r = Regex::new(r",| -> ").expect("Regex is Invalid");
    let x = r
        .splitn(s.as_str(), 4)
        // .inspect(|x| println!("{}", x))
        .map(|x| x.parse::<i32>().unwrap())
        .next_tuple()
        .unwrap();
    x
}


type Grid<'a> = &'a mut [i32];

fn count_overlapping_points(grid: Grid) -> i32 {
    let mut count = 0;
    for i in 0..1000 {
        for j in 0..1000 {
            if grid[i * 1000 + j] > 1 {
                count += 1;
            }
        }
    }

    count
}

fn blit_line(grid: Grid, (x1, y1, x2, y2): Quaduple<i32>) {
    let xdif: i32 = if x2 - x1 > 0 {1} else {-1};
    let ydif: i32 = if y2 - y1 > 0 {1} else {-1};

    let mut x = x1;
    let mut y = y1;

    let mut end = false;
    loop {
        if (x == x2) && (y == y2) {
            end = true;
        }

        let pos = (y * 1000 + x);
        grid[pos as usize] += 1;

        if y != y2 {
            y =  y + ydif;
        }

        if x != x2 {
            x = x + xdif;
        }

        if end {
            break;
        }
    }

}

fn is_non_diagonal((x1, y1, x2, y2): &Quaduple<i32>) -> bool {
    (y1 == y2) || (x1 == x2)
}

fn main() {
    let lines = read_vent_lines("src/input.txt").unwrap();
    let parsed_lines = lines.iter().map(parse_vent_line);

    const size: usize = 1000*1000;

    let grid: Grid = &mut [0; size];

    parsed_lines.inspect(|x| println!("{:?}", x))
       .for_each(|p| blit_line(grid, p));

    for i in 0..10 {
        for j in 0..10 {
            print!(" {}", grid[i * 1000 + j]);
        }

        println!("");
    }

    let int_count = count_overlapping_points(grid);

    // int_count += ds.combinations(2)
    //                .map(|x| do_diag_points_intersect(&x[0], &x[1]))
    //                .count();

    println!("Number of intersections: {}", int_count);
}
