use std::convert::TryInto;

#[cfg(feature = "long-indexes")]
type Index = usize;
#[cfg(not(feature = "long-indexes"))]
type Index = u32;

////////////////////////////////////////////////////////////////////////////////

/// This represents a strongly-typed index into a [`TypedVec`] parameterized
/// with the same `PhantomData`.  It should be zero-cost at runtime.
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct TypedIndex<P>(pub Index, std::marker::PhantomData<*const P>);
impl<P> TypedIndex<P> {
    pub fn new(i: usize) -> Self {
        Self::const_new(i.try_into().unwrap())
    }
    pub const fn const_new(i: Index) -> Self {
        Self(i, std::marker::PhantomData)
    }
    pub const fn empty() -> Self {
        Self::const_new(Index::MAX)
    }
}

impl<P> std::ops::Add<usize> for TypedIndex<P> {
    type Output = Self;
    fn add(self, i: usize) -> Self::Output {
        Self::new((self.0 as usize).checked_add(i).unwrap())
    }
}

impl<P> std::ops::AddAssign<usize> for TypedIndex<P> {
    fn add_assign(&mut self, i: usize) {
        let i: Index = i.try_into().unwrap();
        self.0 = self.0.checked_add(i).unwrap();
    }
}

impl<P> std::cmp::PartialEq<usize> for TypedIndex<P> {
    fn eq(&self, i: &usize) -> bool {
        (self.0 as usize).eq(i)
    }
}

////////////////////////////////////////////////////////////////////////////////

/// This represents a strongly-typed `Vec<T>` which can only be accessed by
/// a [`TypedIndex`] parameterized with the same `PhantomData`, at zero
/// run-time cost.
#[derive(Debug)]
pub struct TypedVec<T, P>(Vec<T>, std::marker::PhantomData<*const P>);

impl<T, P> std::ops::Index<TypedIndex<P>> for TypedVec<T, P> {
    type Output = T;
    fn index(&self, index: TypedIndex<P>) -> &Self::Output {
        self.0.index(index.0 as usize)
    }
}

impl<T, P> std::ops::IndexMut<TypedIndex<P>> for TypedVec<T, P> {
    fn index_mut(&mut self, index: TypedIndex<P>) -> &mut Self::Output {
        self.0.index_mut(index.0 as usize)
    }
}

impl<T, P> std::ops::Deref for TypedVec<T, P> {
    type Target = Vec<T> ;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T, P> TypedVec<T, P> {
    pub fn new() -> Self {
        Self::of(Vec::new())
    }
    pub fn with_capacity(s: usize) -> Self {
        Self::of(Vec::with_capacity(s))
    }
    pub fn of(v: Vec<T>) -> Self {
        Self(v, std::marker::PhantomData)
    }
    pub fn push(&mut self, t: T) -> TypedIndex<P> {
        let i = self.next_index();
        self.0.push(t);
        i
    }
    pub fn next_index(&self) -> TypedIndex<P> {
        TypedIndex::new(self.0.len())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct HullTag {}
pub type HullIndex = TypedIndex<HullTag>;
pub type HullVec<T> = TypedVec<T, HullTag>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct EdgeTag {}
pub type EdgeIndex = TypedIndex<EdgeTag>;
pub type EdgeVec<T> = TypedVec<T, EdgeTag>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct PointTag {}
pub type PointIndex = TypedIndex<PointTag>;
pub type PointVec<T> = TypedVec<T, PointTag>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct ContourTag {}
pub type ContourIndex = TypedIndex<ContourTag>;
pub type ContourVec<T> = TypedVec<T, ContourTag>;

////////////////////////////////////////////////////////////////////////////////

pub const EMPTY_EDGE: EdgeIndex = EdgeIndex::empty();
pub const EMPTY_HULL: HullIndex = HullIndex::empty();
pub const EMPTY_CONTOUR: ContourIndex = ContourIndex::empty();
