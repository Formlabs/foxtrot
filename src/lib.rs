//! `cdt` is a library for calculating
//! [Delaunay](https://en.wikipedia.org/wiki/Delaunay_triangulation) and
//! [constrained Delaunay](https://en.wikipedia.org/wiki/Constrained_Delaunay_triangulation)
//! triangulations.
pub(crate) mod contour;
pub(crate) mod predicates;
pub(crate) mod half;
pub(crate) mod hull;
pub(crate) mod indexes;
pub(crate) mod triangulate;
pub use triangulate::Triangulation;

////////////////////////////////////////////////////////////////////////////////
// Common types for points and strongly-typed vectors
type Point = (f64, f64);

////////////////////////////////////////////////////////////////////////////////
/// Single error type for this library
#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    /// Indicates that a fixed edge is perfectly intersected by a point, which
    /// is not allowed.
    #[error("Point is located on a fixed edge but is not its endpoint")]
    PointOnFixedEdge,

    /// Indicates that [`Triangulation::step`] has been called after
    /// triangulation has been completed
    #[error("There are no more points left to triangulate")]
    NoMorePoints,

    /// Indicates that two fixed edges cross, which is illegal
    #[error("Fixed edges cross each other")]
    CrossingFixedEdge,

    /// Returned when the input is empty
    #[error("input cannot be empty")]
    EmptyInput,

    /// Returned when the input contains invalid floating-point values (which
    /// would break comparisons)
    #[error("input cannot contain NaN or infinity")]
    InvalidInput,

    /// Returned when edge indexes are out-of-bounds in the points array, or
    /// an edge has the same source and destination.
    #[error("edge must index into point array and have different src and dst")]
    InvalidEdge,

    /// Returned when points in the input array are exact duplicates
    #[error("points may not include duplicates")]
    DuplicatePoint,
}

////////////////////////////////////////////////////////////////////////////////
// User-friendly exported functions

/// Triangulates a set of points, returning triangles as triples of indexes
/// into the original points list.  The resulting triangulation has a convex
/// hull.
pub fn triangulate(pts: &[Point]) -> Result<Vec<(usize, usize, usize)>, Error> {
    let mut t = triangulate::Triangulation::new(&pts)?;
    t.run()?;
    Ok(t.triangles().collect())
}

/// Triangulates a set of points with certain fixed edges.  The edges are
/// assumed to form closed boundaries; only triangles within those boundaries
/// will be returned.
pub fn triangulate_with_edges<'a, E>(pts: &[Point], edges: E)
    -> Result<Vec<(usize, usize, usize)>, Error>
    where E: IntoIterator<Item=&'a (usize, usize)> + Copy + Clone
{
    let mut t = triangulate::Triangulation::new_with_edges(&pts, edges)?;
    t.run()?;
    Ok(t.triangles().collect())
}
