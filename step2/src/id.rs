#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Id<T>(pub usize, std::marker::PhantomData<*const T>);
impl<T> Id<T> {
    pub fn new(i: usize) -> Self {
        Id(i, std::marker::PhantomData)
    }
    pub fn empty() -> Self {
        Id::new(0)
    }
}
unsafe impl<T> Sync for Id<T> {}
unsafe impl<T> Send for Id<T> {}
impl <T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::new(self.0)
    }
}
impl <T> Copy for Id<T> {}
