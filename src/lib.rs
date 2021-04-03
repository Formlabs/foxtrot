use itertools::Itertools;
use ordered_float::OrderedFloat;

type Point = (f64, f64);

/// Finds the circumcenter of a triangle
///
/// The result is returned in terms of x-y coordinates, relative to the
/// triangle's point `a` (that is, `a` is the origin of the coordinate system).
/// Hence, the x-y coordinates returned are _not_ absolute; one must add the
/// coordinates of `a` to find the absolute coordinates of the circumcircle.
/// However, this means that the result is frequently more accurate than would
/// be possible if absolute coordinates were returned, due to limited
/// floating-point precision.  In general, the circumradius can be computed
/// much more accurately.
///
/// Based on [Jonathan R Shewchuk's predicates](https://www.ics.uci.edu/~eppstein/junkyard/circumcenter.html)
pub fn circumdelta(a: Point, b: Point, c: Point) -> Point {
    /* Use coordinates relative to point `a' of the triangle. */
    let xba = b.0 - a.0;
    let yba = b.1 - a.1;
    let xca = c.0 - a.0;
    let yca = c.1 - a.1;

    /* Squares of lengths of the edges incident to `a'. */
    let balength = xba * xba + yba * yba;
    let calength = xca * xca + yca * yca;

    /* Calculate the denominator of the formulae. */
    let denominator = 0.5 / geometry_predicates::orient2d(
        [b.0, b.1], [c.0, c.1], [a.0, a.1]);

    /* Calculate offset (from `a') of circumcenter. */
    let xcirca = (yca * balength - yba * calength) * denominator;
    let ycirca = (xba * calength - xca * balength) * denominator;

    (xcirca, ycirca)
}

/// Returns the circumcenter of a triangle with the given points
pub fn circumcenter(a: Point, b: Point, c: Point) -> Point {
    let d = circumdelta(a, b, c);
    (a.0 + d.0, a.1 + d.1)
}

/// Returns the squared circumradius of a triangle with the given points
pub fn circumradius2(a: Point, b: Point, c: Point) -> f64 {
    let d = circumdelta(a, b, c);
    d.0*d.0 + d.1*d.1
}

/// Returns a pseudo-angle in the 0-1 range, without expensive trig functions
pub fn pseudo_angle(a: Point) -> f64 {
    let p = a.0 / (a.0.abs() + a.1.abs());
    (if a.1 > 0.0 {
        3.0 - p
    }  else {
        1.0 + p
    }) / 4.0
}

pub fn distance2(a: Point, b: Point) -> f64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    dx*dx + dy*dy
}

// Re-export with snake_case naming
pub use geometry_predicates::incircle as in_circle;

pub fn triangulate(points: &[Point]) -> Vec<[usize; 3]> {
    let x_bounds = points.iter().map(|p| p.0).minmax().into_option().unwrap();
    let y_bounds = points.iter().map(|p| p.1).minmax().into_option().unwrap();

    let center = ((x_bounds.0 + x_bounds.1) / 2.0,
                  (y_bounds.0 + y_bounds.1) / 2.0);

    // Pick the initial triangle, with
    //  a) the point closest to the center
    //  b) the point closest to a
    //  c) the point with the minimum circumradius
    let a = points.iter()
        .position_min_by_key(
            |q| OrderedFloat(distance2(center, **q)))
        .expect("Could not get initial point");
    let b = points.iter().enumerate()
        .filter(|(j, _)| *j != a)
        .position_min_by_key(
            |(_, p)| OrderedFloat(distance2(points[a], **p)))
        .expect("Could not get second point");
    let c = points.iter().enumerate()
        .filter(|(j, _)| *j != a && *j != b)
        .position_min_by_key(
            |(_, p)| OrderedFloat(circumradius2(points[a], points[b], **p)))
        .expect("Could not get third point");

    let center = circumcenter(points[a], points[b], points[c]);
    let mut points: Vec<Point> = points.iter().cloned().collect();
    points.sort_by_key(|p| OrderedFloat(distance2(center, *p)));
    unimplemented!()
}
