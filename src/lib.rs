type Point = [f64; 2];

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
    let xba = b[0] - a[0];
    let yba = b[1] - a[1];
    let xca = c[0] - a[0];
    let yca = c[1] - a[1];

    /* Squares of lengths of the edges incident to `a'. */
    let balength = xba * xba + yba * yba;
    let calength = xca * xca + yca * yca;

    /* Calculate the denominator of the formulae. */
    let denominator = 0.5 / geometry_predicates::orient2d(b, c, a);

    /* Calculate offset (from `a') of circumcenter. */
    let xcirca = (yca * balength - yba * calength) * denominator;
    let ycirca = (xba * calength - xca * balength) * denominator;

    [xcirca, ycirca]
}

/// Returns the circumcenter of a triangle with the given points
pub fn circumcenter(a: Point, b: Point, c: Point) -> Point {
    let d = circumdelta(a, b, c);
    [a[0] + d[0], a[1] + d[1]]
}

/// Returns the squared circumradius of a triangle with the given points
pub fn circumradius2(a: Point, b: Point, c: Point) -> f64 {
    let d = circumdelta(a, b, c);
    d[0]*d[0] + d[1]*d[1]
}

// Re-export with snake_case naming
pub use geometry_predicates::incircle as in_circle;
