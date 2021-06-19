use arrayvec::ArrayVec;

#[derive(Debug)]
pub struct Id<T>(pub usize, std::marker::PhantomData<*const T>);
impl<T> Id<T> {
    pub fn new(i: usize) -> Self {
        Id(i, std::marker::PhantomData)
    }
    pub fn empty() -> Self {
        Id::new(0)
    }
    pub fn cast<V>(&self) -> Id<V> {
        Id::new(self.0)
    }
}
// Manually implement a bunch of traits to work around an issue with overly
// conservative derives: https://github.com/rust-lang/rust/issues/26925
unsafe impl<T> Sync for Id<T> {}
unsafe impl<T> Send for Id<T> {}
impl <T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::new(self.0)
    }
}
impl <T> Copy for Id<T> {}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Id<T> {}
impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

pub(crate) trait HasId {
    fn append_ids(&self, v: &mut Vec<usize>);
}
impl<T> HasId for Id<T> {
    fn append_ids(&self, v: &mut Vec<usize>) {
        v.push(self.0)
    }
}
impl<T: HasId> HasId for Vec<T> {
    fn append_ids(&self, v: &mut Vec<usize>) {
        for t in self {
            t.append_ids(v);
        }
    }
}
impl<T: HasId, const CAP: usize> HasId for ArrayVec<T, CAP> {
    fn append_ids(&self, v: &mut Vec<usize>) {
        for t in self {
            t.append_ids(v);
        }
    }
}
impl<T: HasId> HasId for Option<T> {
    fn append_ids(&self, v: &mut Vec<usize>) {
        if let Some(s) = self {
            s.append_ids(v);
        }
    }
}
impl HasId for i64 {
    fn append_ids(&self, _v: &mut Vec<usize>) { /* Nothing to do here */ }
}
impl HasId for f64 {
    fn append_ids(&self, _v: &mut Vec<usize>) { /* Nothing to do here */ }
}
impl HasId for &str {
    fn append_ids(&self, _v: &mut Vec<usize>) { /* Nothing to do here */ }
}
impl HasId for bool {
    fn append_ids(&self, _v: &mut Vec<usize>) { /* Nothing to do here */ }
}
