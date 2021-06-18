/*!
`cdt` is a library for calculating
[Delaunay](https://en.wikipedia.org/wiki/Delaunay_triangulation) and
[constrained Delaunay](https://en.wikipedia.org/wiki/Constrained_Delaunay_triangulation)
triangulations.

It is optimized for correctness and speed, using exact predicates to perform
point-in-circle and orientation tests.

# Examples
## Delaunay triangulation
This triangulates a set of four points in a square
```rust
let pts = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
let triangles = cdt::triangulate_points(&pts).unwrap();
assert!(triangles.len() == 2);
for t in triangles {
    println!("{:?} {:?} {:?}", pts[t.0], pts[t.1], pts[t.2])
}
```

## Constrained Delaunay triangulation
This triangulates an inner and outer square
```rust
let pts = vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0),
               (0.2, 0.2), (0.8, 0.2), (0.8, 0.8), (0.2, 0.8)];
let triangles = cdt::triangulate_contours(&pts,
        &[vec![0, 1, 2, 3, 0], vec![4, 5, 6, 7, 4]])
    .unwrap();
for t in triangles {
    println!("{:?} {:?} {:?}", pts[t.0], pts[t.1], pts[t.2])
}
```

# Crate features
By default, the library uses `u32` indexes for internal data structures,
to improve performance.  If you are planning to triangulate more than 500M
points in a single pass, you should enable the `long-indexes` feature.
*/

#![warn(missing_docs)]
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
    /// is not allowed.  The variable is the index of the erroneous point.
    #[error("Point is located on a fixed edge but is not its endpoint")]
    PointOnFixedEdge(usize),

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

    /// Returned when the last point in a contour does not match the start
    #[error("contours must be closed")]
    OpenContour,

    /// Returned when the input has fewer than 3 points
    #[error("too few points")]
    TooFewPoints,

    /// Returned when the input does not have a valid seed point
    #[error("could not find initial seed")]
    CannotInitialize,

    /// This indicates a logic error in the crate, but it happens occasionally
    #[error("escaped wedge when searching fixed edge")]
    WedgeEscape,
}

////////////////////////////////////////////////////////////////////////////////
// User-friendly exported functions

/// Triangulates a set of points, returning triangles as triples of indexes
/// into the original points list.  The resulting triangulation has a convex
/// hull.
pub fn triangulate_points(pts: &[Point]) -> Result<Vec<(usize, usize, usize)>, Error> {
    let t = Triangulation::build(&pts)?;
    Ok(t.triangles().collect())
}

/// Triangulates a set of contours, given as indexed paths into the point list.
/// Each contour must be closed (i.e. the last point in the contour must equal
/// the first point), otherwise [`Error::OpenContour`] will be returned.
pub fn triangulate_contours<V>(pts: &[Point], contours: &[V])
    -> Result<Vec<(usize, usize, usize)>, Error>
    where for<'b> &'b V: IntoIterator<Item=&'b usize>
{
    let t = Triangulation::build_from_contours(&pts, contours)?;
    Ok(t.triangles().collect())
}

/// Triangulates a set of points with certain fixed edges.  The edges are
/// assumed to form closed boundaries; only triangles within those boundaries
/// will be returned.
pub fn triangulate_with_edges<'a, E>(pts: &[Point], edges: E)
    -> Result<Vec<(usize, usize, usize)>, Error>
    where E: IntoIterator<Item=&'a (usize, usize)> + Copy + Clone
{
    let t = Triangulation::build_with_edges(&pts, edges)?;
    Ok(t.triangles().collect())
}

/// Given a set of points and edges which are known to panic, figures out the
/// max number of save steps, then saves an SVG right before the panic occurs
pub fn save_debug_panic<'a, E>(pts: &[Point], edges: E, filename: &str)
    -> std::io::Result<()>
    where E: IntoIterator<Item=&'a (usize, usize)> + Copy + Clone + std::panic::UnwindSafe
{
    let mut safe_steps = 0;
    loop {
        let result = std::panic::catch_unwind(move || {
            let mut t = Triangulation::new_with_edges(pts, edges)
                .expect("Could not build CDT triangulation");
            for _ in 0..safe_steps {
                t.step().expect("Step failed");
            }
        });
        if result.is_ok() {
            safe_steps += 1;
        } else {
            safe_steps -= 1;
            break;
        }
    }

    // This will still panic if we can't *construct* the initial triangulation
    let mut t = Triangulation::new_with_edges(pts, edges)
        .expect("Could not build CDT triangulation");
    for _ in 0..safe_steps {
        t.step().expect("Step failed");
    }
    t.save_debug_svg(filename)
}
