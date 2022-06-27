use std::ops::{Deref, DerefMut};

/// Wrapper that indicates a shared reference should be injected.
///
/// Multiple functions can have exclusive references to the same type.
/// You can think of this as `&T`.
///
/// ```ignore
/// #[get("/inject")]
/// fn inject(val: Inject<i32>) -> impl Respond {
///     val.to_string()
/// }
/// ```
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

/// Wrapper that indicates an exclusive reference should be injected.
///
/// Only one function may have a mutable reference to this value.
/// You can think of this as `&mut T`. Any calls that try to get a [`InjectMut`] will block until
/// it's available.
///
/// ```ignore
/// #[get("/inject-mut")]
/// fn inject_mut(mut val: InjectMut<i32>) -> impl Respond {
///     *val = 77;
/// }
/// ```
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
