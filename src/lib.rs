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

pub mod predicates;
pub mod half;
pub mod util;
pub mod sweepcircle;
pub mod sweepline;

const CHECK_INVARIANTS: bool = true;
