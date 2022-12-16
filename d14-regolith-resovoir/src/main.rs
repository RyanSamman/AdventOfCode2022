use std::{collections::HashMap, cmp::{min, max}};

use split_iter::Splittable;

type Pos = (i32, i32);
type Line = (Pos, Pos);

#[derive(Clone, Copy)]
enum Obstacle {
    Air,
    Rock,
    Sand,
}

struct Cave {
    sand_spawner: Pos,
    obstacle: HashMap<Pos, Obstacle>,
    void: i32,
}

impl Cave {
    fn build(rock_paths: Vec<Line>) -> Self {

        let (vertical, horizontal) = rock_paths.into_iter()
            .split(|((_x1, y1), (_x2, y2))| y1 == y2);

        let mut void = 0;
        let mut obstacle = HashMap::new();

        for ((x1, y1), (x2, y2)) in horizontal {
            assert_eq!(y1, y2);

            void = max(y1, void);

            for x in min(x1, x2)..=max(x1, x2) {
                obstacle.insert((x, y1), Obstacle::Rock);
            }
        }


        for ((x1, y1), (x2, y2)) in vertical {
            assert_eq!(x1, x2);
            for y in min(y1, y2)..=max(y1, y2) {
                void = max(y, void);
                obstacle.insert((x1, y), Obstacle::Rock);
            }
        }


        let sand_spawner = (500,0);

        Self {
            sand_spawner,
            obstacle,
            void,
        }
    }

    fn is_below_void(&self, (_, y): Pos) -> bool {
        y >= self.void
    }

    fn tick_a(&mut self) -> bool {
        let mut sand = self.sand_spawner;

        let mut moving = true;
        while moving {
            if self.is_below_void(sand) {
                return false;
            }

            let down = (sand.0, sand.1 + 1);
            let left = (sand.0 - 1, sand.1 + 1);
            let right = (sand.0 + 1, sand.1 + 1);

            if !self.contains_obstacle(down) {
                sand = down;
            } else if !self.contains_obstacle(left) {
                sand = left;
            } else if !self.contains_obstacle(right) {
                sand = right;
            } else {
                moving = false;
            }
        }

        self.obstacle.insert(sand, Obstacle::Sand);

        return true;
    }

    fn tick_b(&mut self) -> bool {
        let mut sand = self.sand_spawner;

        let mut moving = true;
        while moving {
            if self.is_below_void((sand.0, sand.1)) {
                moving = false;
            }

            let down = (sand.0, sand.1 + 1);
            let left = (sand.0 - 1, sand.1 + 1);
            let right = (sand.0 + 1, sand.1 + 1);

            if !self.contains_obstacle(down) {
                sand = down;
            } else if !self.contains_obstacle(left) {
                sand = left;
            } else if !self.contains_obstacle(right) {
                sand = right;
            } else {
                moving = false;
            }
        }

        self.obstacle.insert(sand, Obstacle::Sand);

        return sand != self.sand_spawner
    }

    fn contains_obstacle(&self, pos: Pos) -> bool {
        match self.obsticale_at(pos) {
            Obstacle::Air => false,
            _ => true,
        }
    }

    fn obsticale_at(&self, pos: Pos) -> Obstacle {
        *self.obstacle.get(&pos).unwrap_or(&Obstacle::Air)
    }

    fn visualize(&self, (x1, y1): Pos, (x2, y2): Pos) {

        for y in y1..=y2 {
            for x in x1..=x2 {
                if (x, y) == self.sand_spawner {
                    print!("+");
                } else if self.is_below_void((x, y-2)) {
                    print!("V");
                } else {
                    let c = match self.obsticale_at((x,y)) {
                        Obstacle::Air => '.',
                        Obstacle::Sand => 'o',
                        Obstacle::Rock => '#',
                    };

                    print!("{c}");
                }
            }
            println!();
        }

    }

}

fn parse_lines(input: &str) -> Vec<Line> {
    let mut pairs = Vec::new();

    for line in input.lines() {
        let positions = line
            .split(" -> ")
            .map(|s| s.splitn(2, ','))
            .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
            .map(|(s1, s2)| (s1.parse::<i32>().unwrap(), s2.parse::<i32>().unwrap()))
            .collect::<Vec<Pos>>();

        let i1 = positions.iter().cloned();
        let mut i2 = positions.iter().cloned();
        i2.next();

        pairs.extend(i1.zip(i2));
    }

    pairs
}

fn part_a(input: &str) {
    let paths = parse_lines(input);

    let mut cave = Cave::build(paths);

    let mut sand_pieces = 0;

    loop {
        sand_pieces += 1;
        //cave.visualize((500 - 10, 0), (500+10, 14));
        if !cave.tick_b() {
            break;
        }
    }

    println!("{sand_pieces}");
}

fn main() {
    let debug = false;
    let input = if debug {include_str!("./test-input.txt")} else {include_str!("./input.txt")};
    part_a(input);
}
