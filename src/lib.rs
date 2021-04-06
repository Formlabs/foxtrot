use std::num::NonZeroUsize;

type Point = (f64, f64);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct PointIndex(usize);

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
