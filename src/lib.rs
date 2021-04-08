type Point = (f64, f64);

safe_index::new! {
    PointIndex,
    map: PointVec with iter: PointIter
}
impl Default for PointIndex {
    fn default() -> Self { Self::new(0) }
}

safe_index::new! {
    EdgeIndex,
    map: EdgeVec with iter: EdgeIter
}
impl Default for EdgeIndex {
    fn default() -> Self { Self::new(0) }
}

pub mod predicates;
pub mod triangulate;
pub mod half;
pub mod hull;
pub mod util;

const CHECK_INVARIANTS: bool = false;
