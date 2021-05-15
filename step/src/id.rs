use std::any::Any;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub struct Id<T>(usize, PhantomData<*const T>);
impl<T> Id<T> {
    pub const fn new(u: usize) -> Self {
        Self(u, PhantomData)
    }

    pub(crate) fn raw(&self) -> usize {
        self.0
    }
}

// TODO move to a different file?
pub(crate) fn dynamic_cast<T, U>(t: &T) -> &U
where
    T: Any,
    U: Any,
{
    let any: &dyn Any = t;
    any.downcast_ref::<U>()
        .expect("dynamic casting failed, T and U are not the same type!")
}
