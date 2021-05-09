use crate::Point;

pub fn in_circle(a: Point, b: Point, c: Point, d: Point) -> f64 {
    geometry_predicates::incircle([a.0, a.1], [b.0, b.1], [c.0, c.1], [d.0, d.1])
}

pub fn orient2d(a: Point, b: Point, c: Point) -> f64 {
    geometry_predicates::orient2d([a.0, a.1], [b.0, b.1], [c.0, c.1])
}

/// Checks whether the angle given by a-b-c is acute, returning a positive
/// value if that is the case.
pub fn acute(a: Point, b: Point, c: Point) -> f64 {
    let x_ba = a.0 - b.0;
    let y_ba = a.1 - b.1;

    let x_bc = c.0 - b.0;
    let y_bc = c.1 - b.1;

    // Dot product
    x_ba * x_bc + y_ba * y_bc
}

/// Returns a pseudo-angle in the 0-1 range, without expensive trig functions
///
/// The angle has the following shape:
/// ```text
///              0.25
///               ^ y
///               |
///               |
///   0           |           x
///   <-----------o-----------> 0.5
///   1           |
///               |
///               |
///               v
///              0.75
/// ```
pub fn pseudo_angle(a: Point) -> f64 {
    let p = a.0 / (a.0.abs() + a.1.abs());
    1.0 - (if a.1 > 0.0 {
        3.0 - p
    }  else {
        1.0 + p
    }) / 4.0
}

pub fn centroid(a: Point, b: Point, c: Point) -> Point {
    ((a.0 + b.0 + c.0) / 3.0, (a.1 + b.1 + c.1) / 3.0)
}

pub fn distance2(a: Point, b: Point) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx*dx + dy*dy
}
