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
