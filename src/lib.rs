use std::num::NonZeroUsize;

type Point = (f64, f64);

safe_index::new! {
    PointIndex,
    map: PointVec with iter: PointIter
}
impl Default for PointIndex {
    fn default() -> Self { Self::new(0) }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TriangleIndex(usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EdgeIndex(NonZeroUsize);
impl Default for EdgeIndex {
    fn default() -> Self { EdgeIndex(NonZeroUsize::new(1).unwrap()) }
}

pub mod predicates;
pub mod triangulate;
pub mod half;
pub mod hull;
