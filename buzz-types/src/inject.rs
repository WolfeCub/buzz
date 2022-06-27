use std::ops::{Deref, DerefMut};

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

pub struct InjectMut<'a, T> {
    inner: &'a mut T,
}

impl<'a, T> InjectMut<'a, T> {
    pub fn new(inner: &'a mut T) -> Self {
        Self {
            inner,
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner
    }
}

impl<'a, T> Deref for InjectMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T> DerefMut for InjectMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}
