// TODO: move these to crate flags
const CHECK_INVARIANTS: bool = false;
const SAVE_DEBUG_SVGS: bool = false;

////////////////////////////////////////////////////////////////////////////////

pub mod contour;
pub mod predicates;
pub mod half;
pub mod util;
pub mod triangulate;
pub mod hull;

////////////////////////////////////////////////////////////////////////////////
// Common types for points and strongly-typed vectors
type Point = (f64, f64);

safe_index::new! { PointIndex, map: PointVec with iter: PointIter }
safe_index::new! { EdgeIndex, map: EdgeVec with iter: EdgeIter }
safe_index::new! { HullIndex, map: HullVec with iter: HullIter }

////////////////////////////////////////////////////////////////////////////////
// Single error type for the whole crate
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Point is located on a fixed edge but is not its endpoint")]
    PointOnFixedEdge,

    #[error("There are no more points left to triangulate")]
    NoMorePoints,

    #[error("Fixed edges cross each other")]
    CrossingFixedEdge,

    #[error("input cannot be empty")]
    EmptyInput,

    #[error("input cannot contain NaN or infinity")]
    InvalidInput,

    #[error("edge must index into point array and have different src and dst")]
    InvalidEdge,
}

////////////////////////////////////////////////////////////////////////////////
// User-friendly exported functions
pub fn triangulate(pts: &[Point]) -> Result<Vec<(usize, usize, usize)>, Error> {
    let mut t = triangulate::Triangulation::new(&pts)?;
    t.run()?;
    Ok(t.triangles().collect())
}
