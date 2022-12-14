use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use itertools::Itertools;

fn parse_heightmap(input: &str) -> Heightmap {
    let lines = input.lines().collect_vec();

    let rows = lines.len() as i32;
    let cols = lines[0].len() as i32;

    let (a_i32, z_i32) = ('a' as i32, 'z' as i32);

    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut char_to_height = |c: char, row: i32, col: i32| -> i32 {
        if c == 'S' {
            start = (row, col);
            a_i32
        } else if c == 'E' {
            end = (row, col);
            z_i32 - a_i32
        } else if c.is_ascii_lowercase() {
            c as i32 - a_i32
        } else {
            panic!("Invalid character")
        }
    };

    let grid: HashMap<(i32, i32), i32> = lines
        .iter()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(move |(col, c)| ((row as i32, col as i32), c))
        })
        .flatten()
        .map(|((r, c), h)| ((r, c), char_to_height(h, r, c)))
        .collect();

    Heightmap {
        grid,
        start,
        end,
        rows,
        cols,
    }
}

type Pos = (i32, i32);

struct Heightmap {
    grid: HashMap<Pos, i32>,
    start: Pos,
    end: Pos,
    rows: i32,
    cols: i32,
}

impl Heightmap {
    fn find_shortest_path(&self, start: Pos, end: Pos) -> usize {
        let mut visited: HashSet<Pos> = HashSet::new();
        let mut queue: Vec<(Pos, usize)> = Vec::new();

        queue.push((start, 0));
        visited.insert(start);

        while !queue.is_empty() {
            let (node, dist) = queue.remove(0);

            if node == end {
                return dist;
            }

            let unvisited_paths = [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .map(|(r, c)| (r + node.0, c + node.1))
                .filter(|p| self.grid.contains_key(p))
                .filter(|p| !visited.contains(p))
                .filter(|p| self.grid[p] - 1 <= self.grid[&node])
                .map(|p| (p, dist + 1))
                .collect_vec();

            unvisited_paths.iter().for_each(|(p, _)| {
                visited.insert(*p);
            });

            queue.extend(unvisited_paths);
        }

        usize::MAX
    }

    fn lowest_points(&self) -> Vec<Pos> {
        self.grid
            .iter()
            .filter(|&(p, &h)| h == 0)
            .map(|(&p, h)| p)
            .collect_vec()
    }

    fn blit_grid(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                match (i, j) {
                    x if x == self.start => print!("S  "),
                    x if x == self.end => print!("E  "),
                    _ => print!("{:<3}", self.grid[&(i, j)]),
                }
            }
            println!();
        }
    }
}

fn solve_a(input: &str) -> String {
    let hm = parse_heightmap(input);

    hm.blit_grid();
    hm.find_shortest_path(hm.start, hm.end).to_string()
}

fn solve(input: &str) -> String {
    let hm = parse_heightmap(input);

    hm.lowest_points()
        .into_iter()
        .map(|p| hm.find_shortest_path(p, hm.end))
        .min()
        .unwrap()
        .to_string()
}

fn test_solution_a() {
    let test_input = include_str!("./test-input.txt");
    assert_eq!(solve(test_input), 31.to_string());
}

fn test_solution2_a() {
    let test_input = include_str!("./input.txt");
    assert_eq!(solve(test_input), 31.to_string());
}

fn test_solution_b() {
    let test_input = include_str!("./test-input.txt");
    assert_eq!(solve(test_input), 29.to_string());
}

fn test_solution2_b() {
    let test_input = include_str!("./input.txt");
    assert_eq!(solve(test_input), 31.to_string());
}

fn main() {
    test_solution_b();
    test_solution2_b();
}
