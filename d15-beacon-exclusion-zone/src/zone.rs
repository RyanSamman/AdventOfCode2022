use std::collections::HashSet;

use crate::{
    beacon::{Sensor, Pos},
    util::valid_pos,
};

#[derive(Debug)]
pub struct Zone {
    sensors: Vec<Sensor>,
}

impl Zone {
    pub fn new(sensors: Vec<Sensor>) -> Self {
        Self { sensors }
    }

    pub fn is_within_zone(&self, p: Pos) -> bool {
        self.sensors.iter().any(|b| b.within_radius(p))
    }

    // Is within a zone, but is not where a beacon already is
    pub fn can_contain_distress_beacon(&self, p: Pos) -> bool {
        self.is_within_zone(p) && !self.sensors.iter().any(|s| s.beacon_pos == p)
    }

    pub fn find_distress_beacon_slow(&self, range: i32) -> Pos {
        for x in 0..=range {
            for y in 0..=range {
                let p = (x, y);
                if !self.is_within_zone(p) {
                    return p;
                }
            }
        }

        (-1, -1)
    }

    // Could check the boundaries of the manhattan radii for a much faster algorithm
    // there is only one place where the beacon could be on the map,
    // right outside the radii of one of the sensors

    pub fn find_distress_beacon(&self, range: i32) -> Pos {


        let boundaries = self
            .sensors
            .iter()
            .map(Sensor::boundary_points)
            .flatten()
            .collect::<HashSet<Pos>>();

        for pos in boundaries {
            if valid_pos(range, pos) && !self.is_within_zone(pos) {
                return pos;
            }
        }

        (-1, -1)
    }
}
