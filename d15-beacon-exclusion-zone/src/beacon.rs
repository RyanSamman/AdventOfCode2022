use crate::util::manhattan_distance;

pub type Pos = (i32, i32);

#[derive(Debug)]
pub struct Sensor {
    pub sensor_pos: Pos,
    // closest beacon position
    pub beacon_pos: Pos,

    // manhattan distance radius where the closest beacon is
    pub radius: i32,
}

impl Sensor {
    pub fn new(sensor_pos: Pos, beacon_pos: Pos) -> Self {
        let radius = manhattan_distance(sensor_pos, beacon_pos);
        Self { sensor_pos, beacon_pos, radius }
    }

    pub fn within_radius(&self, p: Pos) -> bool {
        manhattan_distance(p, self.sensor_pos) <= self.radius
    }

    // Points on the outskirts of the sensor radius
    pub fn boundary_points(&self) -> Vec<Pos> {
        let mut points = Vec::new();
        let (sx, sy) = self.sensor_pos;
        let r = self.radius + 1;
        for offset in 0..r {
            let p1 = (sx + offset, sy + r - offset);
            let p2 = (sx + r - offset, sy - offset);
            let p3 = (sx - offset, sy - r + offset);
            let p4 = (sx - r + offset, sy + offset);

           points.extend([p1, p2, p3, p4]);
        }

        points.sort();

        points
    }
}
