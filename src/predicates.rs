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
