type Point = (f64, f64);

safe_index::new! {
    PointIndex,
    map: PointVec with iter: PointIter
}

safe_index::new! {
    EdgeIndex,
    map: EdgeVec with iter: EdgeIter
}

safe_index::new! {
    HullIndex,
    map: HullVec with iter: HullIter
}

pub mod contour;
pub mod predicates;
pub mod half;
pub mod util;
pub mod triangulate;
pub mod hull;

const CHECK_INVARIANTS: bool = true;
const SAVE_DEBUG_SVGS: bool = false;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Point is located on a fixed edge but is not its endpoint")]
    PointOnFixedEdge,

    #[error("There are no more points left to triangulate")]
    NoMorePoints,

    #[error("Fixed edges cross each other")]
    CrossingFixedEdge,
}
