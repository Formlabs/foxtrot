safe_index::new! { PointIndex, map: PointVec with iter: PointIter }
safe_index::new! { EdgeIndex, map: EdgeVec with iter: EdgeIter }
safe_index::new! { HullIndex, map: HullVec with iter: HullIter }

pub const EMPTY_EDGE: EdgeIndex = EdgeIndex { val: std::usize::MAX };
pub const EMPTY_HULL: HullIndex = HullIndex { val: std::usize::MAX };

pub const POINT_INDEX_ZERO: PointIndex = PointIndex { val: 0 };
pub const POINT_INDEX_ONE: PointIndex = PointIndex { val: 1 };
