//! Module containing various utility functions.


use std::mem::transmute;


/// If the specified 32-bit float is a signaling NaN, make it a quiet NaN.
///
/// Based on [`f32::from_bits()`](https://github.com/rust-lang/rust/pull/39271/files#diff-f60977ab00fd9ea9ba7ac918e12a8f42R1279)
pub fn designalise_f32(f: f32) -> f32 {
    const EXP_MASK: u32 = 0x7F800000;
    const QNAN_MASK: u32 = 0x00400000;
    const FRACT_MASK: u32 = 0x007FFFFF;

    let mut f: u32 = unsafe { transmute(f) };

    if f & EXP_MASK == EXP_MASK && f & FRACT_MASK != 0 {
        // If we have a NaN value, we
        // convert signaling NaN values to quiet NaN
        // by setting the the highest bit of the fraction
        f |= QNAN_MASK;
    }

    unsafe { transmute(f) }
}

/// If the specified 64-bit float is a signaling NaN, make it a quiet NaN.
///
/// Based on [`f64::from_bits()`](https://github.com/rust-lang/rust/pull/39271/files#diff-2ae382eb5bbc830a6b884b8a6ba5d95fR1171)
pub fn designalise_f64(f: f64) -> f64 {
    const EXP_MASK: u64 = 0x7FF0000000000000;
    const QNAN_MASK: u64 = 0x0001000000000000;
    const FRACT_MASK: u64 = 0x000FFFFFFFFFFFFF;

    let mut f: u64 = unsafe { transmute(f) };

    if f & EXP_MASK == EXP_MASK && f & FRACT_MASK != 0 {
        // If we have a NaN value, we
        // convert signaling NaN values to quiet NaN
        // by setting the the highest bit of the fraction
        f |= QNAN_MASK;
    }

    unsafe { transmute(f) }
}

/// Check whether the slice is properly aligned in memory for reading
/// a `T`.
pub(crate) fn check_align<T>(v: &[u8]) -> Result<(), super::error::Error> {
    let align_offset = v.as_ptr() as usize % ::std::mem::align_of::<T>();
    if align_offset != 0 {
        return Err(super::error::Error::Unaligned{ offset: ::std::mem::align_of::<T>() - align_offset});
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::super::error::Error::Unaligned;
    use super::super::to_bytes::guarded_transmute_to_bytes_many;
    use super::check_align;
    use std::ptr;

    #[cfg(target_endian = "little")]
    #[test]
    fn test_check_align() {
        // this is 4-byte aligned
        let data: &[u32] = &[0x000a0005];
        
        let v: &[u8] = unsafe { guarded_transmute_to_bytes_many(data) };
        check_align::<u16>(v).expect("aligned");
        // it's safe to read
        assert_eq!(unsafe { ptr::read(v.as_ptr() as *const u16) }, 5);
        
        let v2 = &v[1..];
        assert_eq!(check_align::<u16>(v2), Err(Unaligned{ offset: 1}));
        // must use `read_unaligned` here or it's UB
        assert_eq!(unsafe { ptr::read_unaligned(v2.as_ptr() as *const u16) }, 2560);

        check_align::<u32>(v).expect("aligned");
        // also safe to read
        assert_eq!(unsafe { ptr::read(v.as_ptr() as *const u32) }, 0x000a0005);

        let v3 = &v[1..];
        assert_eq!(check_align::<u32>(v3), Err(Unaligned{ offset: 3}));
        // not safe to read in any way (out of bounds)

        let v4 = &v[4..];
        check_align::<u32>(v4).expect("aligned");
        // aligned but not safe to read (out of bounds)
    }

    #[cfg(target_endian = "big")]
    #[test]
    fn test_check_align() {
        // this is 4-byte aligned
        let data: &[u32] = &[0x000a0005];
        
        let v: &[u8] = unsafe { guarded_transmute_to_bytes_many(data) };
        check_align::<u16>(v).expect("aligned");
        // it's safe to read
        assert_eq!(unsafe { ptr::read(v.as_ptr() as *const u16) }, 10);
        
        let v2 = &v[1..];
        assert_eq!(check_align::<u16>(v2), Err(Unaligned{ offset: 1}));
        // must use `read_unaligned` here or it's UB
        assert_eq!(unsafe { ptr::read_unaligned(v2.as_ptr() as *const u16) }, 2560);

        check_align::<u32>(v).expect("aligned");
        // also safe to read
        assert_eq!(unsafe { ptr::read(v.as_ptr() as *const u32) }, 0x000a0005);

        let v3 = &v[1..];
        assert_eq!(check_align::<u32>(v3), Err(Unaligned{ offset: 3}));
        // not safe to read in any way (out of bounds)

        let v4 = &v[4..];
        check_align::<u32>(v4).expect("aligned");
        // aligned but not safe to read (out of bounds)
    }
}