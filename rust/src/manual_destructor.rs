use std::ops::{Deref, DerefMut};

/// A wrapper for T that allows a static variable fall out of scope on demand. Somewhat like making a type nullable.
/// Panics if access is attempted after destruction
pub struct ManualDestructor<T> {
    value: Option<T>
}

impl<T> ManualDestructor<T> {
    pub fn new(value: T) -> ManualDestructor<T> {
        ManualDestructor{
            value: Some(value)
        }
    }

    /// Force T to go out of scope, thus causing rust to drop all of its allocated memory and resources
    pub fn destroy(&mut self)  {
        self.value = None;
    }
}

impl<T> Deref for ManualDestructor<T> {
    type Target = T;
    /// Makes it easy access at the cost of panic risk. Do not call after destroy
    fn deref(&self) -> &Self::Target {
        &self.value.as_ref().expect("Attempt to access ManualDestructor after destruction!")
    }
}

impl<T> DerefMut for ManualDestructor<T> {
    /// Makes it easy access mutably at the cost of panic risk. Do not call after destroy
    fn deref_mut(&mut self) -> &mut Self::Target {
        let mut v = self.value.as_mut().expect("Attempt to access ManualDestructor after destruction!");
        v
    }
}