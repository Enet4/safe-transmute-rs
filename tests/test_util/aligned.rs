use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy)]
#[repr(align(2))]
pub struct Aligned2<T>(pub T);

impl<T: AsRef<[u8]>> AsRef<[u8]> for Aligned2<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T: AsMut<[u8]>> AsMut<[u8]> for Aligned2<T> {
    fn as_mut(&self) -> &mut [u8] {
        self.0.as_mut()
    }
}

impl<T> Deref for Aligned2<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Aligned2<T> {
    fn deref(&mut self) -> &mut T {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
pub struct Aligned4<T>(pub T);

impl<T: AsRef<[u8]>> AsRef<[u8]> for Aligned4<T> {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<T> Deref for Aligned4<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Aligned4<T> {
    fn deref(&mut self) -> &mut T {
        &self.0
    }
}
