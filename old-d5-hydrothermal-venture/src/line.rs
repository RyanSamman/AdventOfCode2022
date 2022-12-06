mod line {
    // (incomplete) Horrible approach causes you to need to:
    // 1. divide (which is a slow operation)
    // 2. may divide by 0
    // 3. Lots of edge cases
    use approx::{assert_relative_ne, relative_eq};


    fn compute_slope(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        (y2 - y1) / (x2 - x1)
    }

    fn compute_y_intercept(x: f64, y: f64, m: f64) -> f64 {
        y - m * x
    }

    fn compute_y(m: f64, x: f64, c: f64) -> f64 {
        m * x + c
    }

    fn is_point_on_line((x, y): &Point) -> bool {
        let computed_y = compute_y(x.clone());
        relative_eq!(y.clone(), computed_y)
    }

    fn is_point_on_bounded_line(&self, p: &Point) -> bool {
        if !self.is_point_on_line(p) {
            return false;
        }

        let (x, y) = p;
        let (x1, y1) = self.lower_bound;
        let (x2, y2) = self.upper_bound;

        let (lx, ux) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (ly, uy) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

        (lx..ux).contains(x) && (ly..uy).contains(y)
    }

    fn intersects(line: Line) -> bool {
        let dy_int = self.y_intercept - line.y_intercept;
        let dm = self.slope - line.slope;
        // if slopes are the same:
        if relative_eq!(dm, 0.0) {
            // if both y-int are equal
            if relative_eq!(dy_int, 0.0) {
                // same line
                return true;
            } else {
                // Parelell line
                return false;
            }
        }

        // Intersects at point x_intersect
        let x_intersect = dy_int / dm;
        let y_intersect = self.compute_y(x_intersect);

        self.is_point_on_bounded_line(&(x_intersect, y_intersect))
    }
}
