// I realized a much faster and simpler method that takes advantage of the horizontal/verticality...
// So this isn't needed, but it was a good read ;(

fn is_non_diagonal((x1, y1, x2, y2): &Quaduple<i32>) -> bool {
    (y1 == y2) || (x1 == x2)
}

// Calculate the direction of the rotation created by:
//
//           B
//        u
//                 C
//      A     v
//
// creating a matrix [u v]
// would represent the transformation of another vector in the direction BC (or the perpendicular projection of v onto u)
// it's determinant's sign would tell us whether the orientation has been changed
//
// If 0, then the three numbers are colinear. (scale?)
// If >0, then the transformation moves it clockwise
// If <0, then the transformation moves it anticlockwise
fn transformation_direction(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> i32 {
    // u
    let ux = p2.0 - p1.0;
    let uy = p2.1 - p1.1;

    let vx = p3.0 - p1.0;
    let vy = p3.1 - p1.1;

    // ux vx
    // uy vy
    (ux * vy) - (vx * uy)
}

// Checks if p3 (which must be colinear to p1 and p2) lies within p1 and p2.
fn is_on_line(p1: (i32, i32), p2: (i32, i32), (x, y): (i32, i32)) -> bool {
    let (x1, x2) = (min(p1.0, p2.0), max(p1.0, p2.0));
    let (y1, y2) = (min(p1.1, p2.1), max(p1.1, p2.1));

    x1 <= x && x <= x2 && y1 <= y && y <= y2
}

fn do_points_intersect(l1: &Quaduple<i32>, l2: &Quaduple<i32>) -> bool {
    let l1p1 = (l1.0, l1.1);
    let l1p2 = (l1.2, l1.3);
    let l2p1 = (l2.0, l2.1);
    let l2p2 = (l2.2, l2.3);

    let d1 = transformation_direction(l1p1, l1p2, l2p1);
    let d2 = transformation_direction(l1p1, l1p2, l2p2);
    let d3 = transformation_direction(l2p1, l2p2, l1p1);
    let d4 = transformation_direction(l2p1, l2p2, l1p2);

    // Check if all points lie opposite to eachother on both lines
    if ((d1 > 0 && d2 < 0) || (d2 > 0 && d1 < 0)) && ((d3 > 0 && d4 < 0) || (d4 > 0 && d3 < 0)) {
        return true;
    }

    // Check if points are on the line

    if d1 == 0 && is_on_line(l1p1, l1p2, l2p1) {
        return true;
    }

    if d2 == 0 && is_on_line(l1p1, l1p2, l2p2) {
        return true;
    }

    if d3 == 0 && is_on_line(l2p1, l2p2, l1p1) {
        return true;
    }

    if d4 == 0 && is_on_line(l2p1, l2p2, l1p2) {
        return true;
    }

    return false;
}
