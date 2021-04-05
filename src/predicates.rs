use crate::Point;

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
    let denominator = 0.5 / orient2d(b, c, a);

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

pub fn in_circle(a: Point, b: Point, c: Point, d: Point) -> f64 {
    geometry_predicates::incircle([a.0, a.1], [b.0, b.1], [c.0, c.1], [d.0, d.1])
}

pub fn orient2d(a: Point, b: Point, c: Point) -> f64 {
    geometry_predicates::orient2d([a.0, a.1], [b.0, b.1], [c.0, c.1])
}
