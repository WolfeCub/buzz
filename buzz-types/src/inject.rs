use std::ops::Deref;

pub struct Inject<'a, T> {
    inner: &'a T,
}

impl<'a, T> Inject<'a, T> {
    pub fn new(inner: &'a T) -> Self {
        Self {
            inner,
        }
    }

    pub fn get(&self) -> &T {
        &self.inner
    }
}

impl<'a, T> Deref for Inject<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
