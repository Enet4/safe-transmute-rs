//! Functions for transmutation *from* a concrete type *to* bytes.


use self::super::PodTransmutable;
use core::mem::size_of;
#[cfg(feature = "std")]
use core::mem::forget;
use core::slice;


/// Transmute a single instance of an arbitrary type into a slice of its bytes.
///
/// # Examples
///
/// An `u32`:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_unchecked;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// unsafe {
///     assert_eq!(transmute_to_bytes_unchecked(&0x0123_4567),
/// # /*
///                &[0x67, 0x45, 0x23, 0x01]);
/// # */
/// #               [0x67, 0x45, 0x23, 0x01].le_to_native::<u32>());
/// }
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_unchecked;
/// #[repr(C)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// unsafe {
///     assert_eq!(transmute_to_bytes_unchecked(&Gene {
///                    x1: 0x42,
///                    x2: 0x69,
///                }),
///                &[0x42, 0x69]);
/// }
/// ```
pub unsafe fn transmute_to_bytes_unchecked<T>(from: &T) -> &[u8] {
    slice::from_raw_parts(from as *const T as *const u8, size_of::<T>())
}

/// Transmute a slice of arbitrary types into a slice of their bytes.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// unsafe {
///     assert_eq!(transmute_to_bytes(&[0x0123u16, 0x4567u16]),
/// # /*
///                &[0x23, 0x01, 0x67, 0x45]);
/// # */
/// #               [0x23, 0x01, 0x67, 0x45].le_to_native::<u16>());
/// }
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_many_unchecked;
/// #[repr(C)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// unsafe {
///     assert_eq!(transmute_to_bytes_many_unchecked(&[Gene {
///                                                        x1: 0x42,
///                                                        x2: 0x69,
///                                                    },
///                                                    Gene {
///                                                        x1: 0x12,
///                                                        x2: 0x48,
///                                                    }]),
///                &[0x42, 0x69, 0x12, 0x48]);
/// }
/// ```
pub unsafe fn transmute_to_bytes_many_unchecked<T>(from: &[T]) -> &[u8] {
    slice::from_raw_parts(from.as_ptr() as *const u8, from.len() * size_of::<T>())
}

/// Transmute a single instance of a POD type into a slice of its bytes.
///
/// # Examples
///
/// An `u32`:
///
/// ```
/// # use safe_transmute::transmute_one_to_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(transmute_one_to_bytes(&0x0123_4567),
/// # /*
///            &[0x67, 0x45, 0x23, 0x01]);
/// # */
/// #           [0x67, 0x45, 0x23, 0x01].le_to_native::<u32>());
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::{PodTransmutable, transmute_one_to_bytes};
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
/// unsafe impl PodTransmutable for Gene {}
///
/// assert_eq!(transmute_one_to_bytes(&Gene {
///                x1: 0x42,
///                x2: 0x69,
///            }),
///            &[0x42, 0x69]);
/// ```
pub fn transmute_one_to_bytes<T: PodTransmutable>(from: &T) -> &[u8] {
    unsafe { transmute_to_bytes_unchecked(from) }
}

/// Transmute a slice of arbitrary types into a slice of their bytes.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::transmute_to_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(transmute_to_bytes(&[0x0123u16, 0x4567u16]),
/// # /*
///            &[0x23, 0x01, 0x67, 0x45]);
/// # */
/// #           [0x23, 0x01, 0x67, 0x45].le_to_native::<u16>());
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::{PodTransmutable, transmute_to_bytes};
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
/// unsafe impl PodTransmutable for Gene {}
///
/// assert_eq!(transmute_to_bytes(&[Gene {
///                                          x1: 0x42,
///                                          x2: 0x69,
///                                      },
///                                      Gene {
///                                          x1: 0x12,
///                                          x2: 0x48,
///                                      }]),
///            &[0x42, 0x69, 0x12, 0x48]);
/// ```
pub fn transmute_to_bytes<T: PodTransmutable>(from: &[T]) -> &[u8] {
    unsafe { transmute_to_bytes_many_unchecked(from) }
}

/// Transmute a slice of arbitrary types into a slice of their bytes.
#[deprecated(since = "0.11.0", note = "use `transmute_to_bytes()` instead")]
pub fn guarded_transmute_to_bytes_pod_many<T: PodTransmutable>(from: &[T]) -> &[u8] {
    transmute_to_bytes(from)
}

/// Transmute a vector of elements of an arbitrary type into a vector of their
/// bytes, using the same memory buffer as the former.
/// 
/// The original nature of the elements in the vector is forgotten. This means
/// that, although this function is memory safe, applying it on a vector with
/// a `Drop` implementation is likely to result in memory leaks and other
/// kinds of misbehavior.
///
/// # Examples
///
/// Some `u16`s:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_vec;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// assert_eq!(transmute_to_bytes_vec(vec![0x0123u16, 0x4567u16]),
/// # /*
///            vec![0x23, 0x01, 0x67, 0x45]);
/// # */
/// #          vec![0x23, 0x01, 0x67, 0x45].le_to_native::<u16>());
/// # }
/// ```
///
/// An arbitrary type:
///
/// ```
/// # use safe_transmute::to_bytes::transmute_to_bytes_vec;
/// #[repr(C)]
/// #[derive(Clone, Copy)]
/// struct Gene {
///     x1: u8,
///     x2: u8,
/// }
///
/// assert_eq!(transmute_to_bytes_vec(vec![Gene {
///                                            x1: 0x42,
///                                            x2: 0x69,
///                                        },
///                                        Gene {
///                                            x1: 0x12,
///                                            x2: 0x48,
///                                        }]),
///            vec![0x42, 0x69, 0x12, 0x48]);
/// ```
#[cfg(feature = "std")]
pub fn transmute_to_bytes_vec<T>(mut from: Vec<T>) -> Vec<u8> {
    unsafe {
        let capacity = from.capacity() * size_of::<T>();
        let len = from.len() * size_of::<T>();
        let ptr = from.as_mut_ptr();
        forget(from);
        Vec::from_raw_parts(ptr as *mut u8, len, capacity)
    }
}

/// Transmute a vector of POD types into a vector of their bytes,
/// using the same memory buffer as the former.
#[cfg(feature = "std")]
#[deprecated(since = "0.11.0", note = "use `transmute_to_bytes_vec()` instead")]
pub fn guarded_transmute_to_bytes_pod_vec<T: PodTransmutable>(from: Vec<T>) -> Vec<u8> {
    transmute_to_bytes_vec(from)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    use super::transmute_to_bytes_vec;

    #[cfg(feature = "std")]
    #[test]
    fn high_align_dealloc_issue_16() {
        #[repr(C)]
        #[repr(align(32))]
        #[derive(Copy, Clone)]
        struct Test {
            value: [u8; 32],
        }

        let values = vec![Test { value: [42; 32] }; 1000];
        let bytes = transmute_to_bytes_vec(values);

        assert_eq!(bytes.len(), 32_000);
        assert!(bytes.iter().all(|&v| v == 42));
    }
}
