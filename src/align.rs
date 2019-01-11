//! Alignment checking primitives.
use crate::error::{Error, UnalignedError};
use crate::PodTransmutable;
use crate::guard::{Guard, PermissiveGuard};
use crate::pod::{transmute_pod_many, transmute_pod_vec};
use core::marker::PhantomData;
use core::mem::{align_of, size_of};
use core::ops::{Deref, DerefMut};

/// Newtype for containers or values with additional alignment guarantees.
///
/// # Formal specification
///
/// Given a generic reference `T` that `Deref`s to a value `V` or a slice
/// `[V]`, a value of `Aligned<T, U>` wraps a value `T` which points to a
/// memory location compatible with the memory alignment required by `U`.
///
/// # Example
///
/// 
/// ```
/// let data = vec![2, 4, 6, 8, 1, 2, 3, 4];
///
/// match Aligned::<_, u32>::check_slice(data) {
///     Ok(data) => {
///         // we're safe to read the vector as
///         // a sequence of `u32`s
///         let words: &[u32] = data.safe_transmute_many_permissive();
///         assert!(words[0] > 0x0204_0608);
///     },
///     Err(_) => {
///         // sorry, can't do that safely
///     }
/// }
/// ```
///
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Aligned<T, U>(T, PhantomData<U>);

impl<T, U, V> Aligned<T, U>
where
    T: Deref<Target = [V]>,
{
    pub fn check_slice(x: T) -> Result<Self, UnalignedError> {
        check_alignment::<V, U>(&*x)?;
        Ok(Aligned(x, PhantomData))
    }
}

impl<T, U, V> Aligned<T, U>
where
    T: Deref<Target = V>,
{
    pub fn check_one(x: T) -> Result<Self, UnalignedError> {
        check_alignment_one::<V, U>(&*x)?;
        Ok(Aligned(x, PhantomData))
    }
    
    pub unsafe fn check_slice_unchecked(x: T) -> Self {
        Aligned(x, PhantomData)
    }
}

impl<T, U> Aligned<T, U> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, U> Aligned<T, U>
where
    T: Deref<Target = [u8]>,
    U: PodTransmutable,
{

    pub fn safe_transmute_many<G: Guard>(&self) -> Result<&[U], Error> {
        unsafe {
            // aligned for `U`, so we're good
            transmute_pod_many::<U, G>(&*self.0)
        }
    }

    pub fn safe_transmute_many_permissive(&self) -> &[U] {
        unsafe {
            // aligned for `U`, so we're good
            transmute_pod_many::<U, PermissiveGuard>(&*self.0)
                .expect("permissive guard should never fail")
        }
    }
}

impl<U> Aligned<Vec<u8>, U>
where
    U: PodTransmutable,
{

    pub fn safe_transmute_vec<G: Guard>(self) -> Result<Vec<U>, Error> {
        unsafe {
            // aligned for `U`, so we're good
            transmute_pod_vec::<U, G>(self.0)
        }
    }

    pub fn safe_transmute_vec_permissive(self) -> Vec<U> {
        unsafe {
            // aligned for `U`, so we're good
            transmute_pod_vec::<U, PermissiveGuard>(self.0)
                .expect("permissive guard should never fail")
        }
    }
}

impl<T, U> Deref for Aligned<T, U>
where
    T: Deref,
{
    type Target = T::Target;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}


impl<T, U> DerefMut for Aligned<T, U>
where
    T: DerefMut,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

/// Check whether the given data slice of `T`s is properly aligned for reading
/// and writing as a slice of `U`s.
///
/// # Errors
///
/// An `Error::Unaligned` error is returned with the number of bytes to discard
/// from the front in order to make the conversion safe from alignment concerns.
pub fn check_alignment<T, U>(data: &[T]) -> Result<(), UnalignedError> {
    // TODO this could probably become more efficient once `ptr::align_offset`
    // is stabilized (#44488)
    let ptr = data.as_ptr();
    check_alignment_ptr::<T, U>(ptr)
}

pub fn check_alignment_one<T, U>(data: &T) -> Result<(), UnalignedError> {
    let ptr = data as *const _;
    check_alignment_ptr::<T, U>(ptr)
}

fn check_alignment_ptr<T, U>(ptr: *const T) -> Result<(), UnalignedError> {
    let offset = ptr as usize % align_of::<U>();
    if offset > 0 {
        // reverse the offset (from "bytes to insert" to "bytes to remove")
        Err(UnalignedError { offset: size_of::<U>() - offset })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transmute_to_bytes, transmute_to_bytes_vec};

    #[test]
    fn test_check_aligned() {
        let data: Vec<u32> = vec![1, 2, 3];
        let bytes: Vec<u8> = transmute_to_bytes_vec(data);
        let mut x: Aligned<_, u32> = Aligned::check_slice(bytes).unwrap();
        check_alignment::<_, u32>(&*x).unwrap();

        let bytes = &mut *x;
        bytes[2] = 5;
    }

    #[test]
    fn test_check_unaligned() {
        let data: &[u32] = &[1, 2, 3];
        let bytes: &[u8] = transmute_to_bytes(data);
        assert_eq!(
            Aligned::<_, u32>::check_slice(&bytes[1..]),
            Err(UnalignedError {
                offset: 3,
            }));
    }
}

