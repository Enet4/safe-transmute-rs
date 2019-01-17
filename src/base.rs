//! Primitives for object and array transmutation.
//!
//! The functions in this module are very unsafe and their use is not
//! recommended unless you *really* know what you are doing.


use self::super::guard::{SingleValueGuard, PermissiveGuard, SingleManyGuard, Guard};
use self::super::error::Error;
use core::mem::size_of;
#[cfg(feature = "std")]
use core::mem::forget;
use core::slice;


/// Convert a byte slice into a single instance of a `Copy`able type.
///
/// The byte slice must have at least enough bytes to fill a single instance of
/// a type, extraneous data is ignored.
///
/// # Safety
///
/// - This function does not perform memory alignment checks. The beginning of
///   the slice data must be properly aligned for accessing the value of type `T`.
/// - The byte data needs to correspond to a valid `T` value.
///
/// Failure to fulfill any of the requirements above may result in undefined
/// behavior.
///
/// # Errors
///
/// An error is returned if the slice does not have enough bytes for a single
/// value `T`.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::base::from_bytes;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(from_bytes::<u32>(&[0x00, 0x00, 0x00, 0x01])?, 0x0100_0000);
/// # */
/// #   assert_eq!(from_bytes::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(), 0x0100_0000);
/// }
/// # }
/// ```
pub unsafe fn from_bytes<T: Copy>(bytes: &[u8]) -> Result<T, Error> {
    SingleManyGuard::check::<T>(bytes)?;
    Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, 1)[0])
}

/// Convert a byte slice into a single instance of a `Copy`able type.
///
/// The byte slice must have exactly the expected number of bytes to fill a
/// single instance of a type, without trailing space.
///
/// # Safety
///
/// - This function does not perform memory alignment checks. The beginning of
///   the slice data must be properly aligned for accessing the value of type `T`.
/// - The byte data needs to correspond to a valid `T` value.
///
/// Failure to fulfill any of the requirements above may result in undefined
/// behavior.
///
/// # Errors
///
/// An error is returned if the slice's length is not equal to the size of a
/// single value `T`.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::base::from_bytes_pedantic;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(from_bytes_pedantic::<u32>(&[0x00, 0x00, 0x00, 0x01])?, 0x0100_0000);
/// # */
/// #   assert_eq!(
/// #       from_bytes_pedantic::<u32>(&[0x00, 0x00, 0x00, 0x01].le_to_native::<u32>()).unwrap(),
/// #       0x0100_0000
/// #   );
/// }
/// # }
/// ```
pub unsafe fn from_bytes_pedantic<T: Copy>(bytes: &[u8]) -> Result<T, Error> {
    SingleValueGuard::check::<T>(bytes)?;
    Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, 1)[0])
}

/// View a byte slice as a slice of an arbitrary type.
///
/// The required byte length of the slice depends on the chosen boundary guard.
/// Please see the [Guard API](../guard/index.html).
///
/// # Safety
///
/// - This function does not perform memory alignment checks. The beginning of
///   the slice data must be properly aligned for accessing vlues of type `T`.
/// - The byte data needs to correspond to a valid contiguous sequence of `T`
///   values. Types `T` with a `Drop` implementation are unlikely to be safe
///   in this regard.
///
/// Failure to fulfill any of the requirements above may result in undefined
/// behavior.
///
/// # Errors
///
/// An error is returned if the slice does not have enough bytes for a single
/// value `T`.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::base::transmute_many;
/// # use safe_transmute::SingleManyGuard;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(
///         transmute_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02])?,
/// # */
/// #   assert_eq!(transmute_many::<u16, SingleManyGuard>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///         &[0x0100, 0x0200]
///     );
/// }
/// # }
/// ```
pub unsafe fn transmute_many<T, G: Guard>(bytes: &[u8]) -> Result<&[T], Error> {
    G::check::<T>(bytes)?;
    Ok(slice::from_raw_parts(bytes.as_ptr() as *const T, bytes.len() / size_of::<T>()))
}

/// View a byte slice as a slice of an arbitrary type.
///
/// The resulting slice will have as many instances of a type as will fit,
/// rounded down. The permissive guard is a no-op, which makes it possible for
/// this function to return a slice directly. It is therefore equivalent to
/// `transmute_many::<_, PermissiveGuard>(bytes).unwrap()`.
///
/// # Safety
///
/// - This function does not perform memory alignment checks. The beginning of
///   the slice data must be properly aligned for accessing vlues of type `T`.
/// - The byte data needs to correspond to a valid contiguous sequence of `T`
///   values. Types `T` with a `Drop` implementation are unlikely to be safe
///   in this regard.
///
/// Failure to fulfill any of the requirements above may result in undefined
/// behavior.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::base::transmute_many_permissive;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(
///         transmute_many_permissive::<u16>(&[0x00, 0x01, 0x00, 0x02]),
/// # */
/// #   assert_eq!(transmute_many_permissive::<u16>(&[0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
///         &[0x0100, 0x0200]
///     );
/// }
/// # }
/// ```
pub unsafe fn transmute_many_permissive<T>(bytes: &[u8]) -> &[T] {
    transmute_many::<_, PermissiveGuard>(bytes).expect("permissive guard should never fail")
}

/// Transform a byte vector into a vector of an arbitrary type.
///
/// The vector's allocated byte buffer (if already allocated) will be reused.
/// The required length of the vector depends on the chosen boundary guard.
/// Please see the [Guard API](../guard/index.html).
///
/// # Safety
///
/// - This function does not perform memory alignment checks. The beginning of
///   the slice data must be properly aligned for accessing vlues of type `T`.
/// - The byte data needs to correspond to a valid contiguous sequence of `T`
///   values. Types `T` with a `Drop` implementation are unlikely to be safe
///   in this regard.
///
/// Failure to fulfill any of the requirements above may result in undefined
/// behavior.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::guard::PermissiveGuard;
/// # use safe_transmute::base::transmute_vec;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(
///         transmute_vec::<u16, PermissiveGuard>(vec![0x00, 0x01, 0x00, 0x02])?,
/// # */
/// # assert_eq!(transmute_vec::<u16, PermissiveGuard>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()).unwrap(),
///         vec![0x0100, 0x0200]
///     );
/// # /*
///     assert_eq!(
///         transmute_vec::<u32, PermissiveGuard>(vec![0x04, 0x00, 0x00, 0x00, 0xED])?,
/// # */
/// # assert_eq!(transmute_vec::<u32, PermissiveGuard>(
/// #                vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()).unwrap(),
///         vec![0x0000_0004]
///     );
///     assert_eq!(transmute_vec::<u16, PermissiveGuard>(vec![0xED]), Ok(vec![]));
/// }
/// # }
/// ```
#[cfg(feature = "std")]
pub unsafe fn transmute_vec<T, G: Guard>(mut bytes: Vec<u8>) -> Result<Vec<T>, Error> {
    G::check::<T>(&bytes)?;
    let ptr = bytes.as_mut_ptr();
    let capacity = bytes.capacity() / size_of::<T>();
    let len = bytes.len() / size_of::<T>();
    forget(bytes);
    Ok(Vec::from_raw_parts(ptr as *mut T, len, capacity))
}

/// Transform a byte vector into a vector of an arbitrary type.
///
/// The vector's allocated byte buffer (if already allocated) will be reused.
///
/// # Safety
///
/// - This function does not perform memory alignment checks. The beginning of
///   the slice data must be properly aligned for accessing vlues of type `T`.
/// - The byte data needs to correspond to a valid contiguous sequence of `T`
///   values. Types `T` with a `Drop` implementation are unlikely to be safe
///   in this regard.
///
/// Failure to fulfill any of the requirements above may result in undefined
/// behavior.
///
/// # Examples
///
/// ```no_run
/// # use safe_transmute::base::transmute_vec_permissive;
/// # include!("../tests/test_util/le_to_native.rs");
/// # fn main() {
/// // Little-endian
/// unsafe {
/// # /*
///     assert_eq!(
///         transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02]),
/// # */
/// # assert_eq!(transmute_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
///         vec![0x0100, 0x0200]
///     );
/// # /*
///     assert_eq!(
///         transmute_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED]),
/// # */
/// # assert_eq!(transmute_vec_permissive::<u32>(vec![0x04, 0x00, 0x00, 0x00, 0xED].le_to_native::<u32>()),
///         vec![0x0000_0004]
///     );
///     assert_eq!(transmute_vec_permissive::<u16>(vec![0xED]), vec![]);
/// }
/// # }
/// ```
#[cfg(feature = "std")]
pub unsafe fn transmute_vec_permissive<T>(bytes: Vec<u8>) -> Vec<T> {
    transmute_vec::<T, PermissiveGuard>(bytes).expect("permissive guard should never fail")
}
