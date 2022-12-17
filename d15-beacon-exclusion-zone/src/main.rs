use beacon::Sensor;
use regex::Regex;
use util::tuning_frequency;

use crate::zone::Zone;
mod beacon;
mod util;
mod zone;

fn solve_a(input: &str, row: i32) -> i32 {
    let re = Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;

    let mut sensors = Vec::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let sensor_pos = (
            caps["sx"].parse::<i32>().unwrap(),
            caps["sy"].parse::<i32>().unwrap(),
        );

        let beacon_pos = (
            caps["bx"].parse::<i32>().unwrap(),
            caps["by"].parse::<i32>().unwrap(),
        );

        let b = Sensor::new(sensor_pos, beacon_pos);

        min_x = min_x.min(sensor_pos.0 - b.radius);
        max_x = max_x.max(sensor_pos.0 + b.radius);

        sensors.push(b);
    }

    println!("{min_x} {max_x}");

    let zone = Zone::new(sensors);

    let mut i = 0;
    for x in min_x..=max_x {
        let p = (x, row);
        if !zone.can_contain_distress_beacon(p) {
            i += 1
        }
    }

    i
}

fn solve_b(input: &str, range: i32) -> i64 {
    let re = Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
    let mut sensors = Vec::new();

    for line in input.lines() {
        let caps = re.captures(line).unwrap();

        let sensor_pos = (
            caps["sx"].parse::<i32>().unwrap(),
            caps["sy"].parse::<i32>().unwrap(),
        );

        let beacon_pos = (
            caps["bx"].parse::<i32>().unwrap(),
            caps["by"].parse::<i32>().unwrap(),
        );

        let b = Sensor::new(sensor_pos, beacon_pos);
        sensors.push(b);
    }

    let zone = Zone::new(sensors);

    tuning_frequency(zone.find_distress_beacon(range))
}

fn main() {
    let debug = false;
    if debug {
        let input = include_str!("./test-input.txt");
        assert_eq!(solve_b(input, 20), 26);
    } else {
        let input = include_str!("./input.txt");
        assert_eq!(solve_b(input, 4000000), 26);
    };
}
