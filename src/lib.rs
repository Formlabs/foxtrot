type Point = (f64, f64);

safe_index::new! {
    PointIndex,
    map: PointVec with iter: PointIter
}

safe_index::new! {
    EdgeIndex,
    map: EdgeVec with iter: EdgeIter
}

pub mod predicates;
pub mod half;
pub mod util;
pub mod sweepcircle;

const CHECK_INVARIANTS: bool = false;
