/// create a new vector that is sure to have an alignment compatible
/// with `T` at creation time.
///
/// Do not modify the vector, or this assurance is gone.
fn aligned_vec<T>(bytes: &[u8]) -> Vec<u8> {
    use std::mem::{align_of, forget, size_of};
    let vec_len_offset = bytes.len() % size_of::<T>();
    let vec_len = bytes.len() / size_of::<T>();
    let capacity = if vec_len_offset > 0 { vec_len + 1 } else { vec_len };
    unsafe {
        let mut v: Vec<T> = Vec::with_capacity(capacity);
        let ptr = v.as_mut_ptr() as *mut u8;
        bytes.as_ptr().copy_to_nonoverlapping(ptr, bytes.len());
        forget(v);
        let vec = Vec::from_raw_parts(ptr, bytes.len(), capacity * size_of::<T>());
        assert_eq!((vec.as_ptr() as usize) % align_of::<T>(), 0);
        assert_eq!(vec.len(), bytes.len());
        assert!(vec.capacity() >= vec.len());
        vec
    }
}

#[test]
fn test_aligned_vec() {
    assert_eq!(aligned_vec::<u16>([].as_ref()), vec![]);
    assert_eq!(aligned_vec::<i32>([].as_ref()), vec![]);
    assert_eq!(aligned_vec::<u64>([].as_ref()), vec![]);

    assert_eq!(aligned_vec::<u64>([0].as_ref()), vec![0]);
    assert_eq!(aligned_vec::<u32>([1, 2].as_ref()), vec![1, 2]);
    assert_eq!(aligned_vec::<u64>([1, 2, 3].as_ref()), vec![1, 2, 3]);
    assert_eq!(aligned_vec::<u64>([0xAA; 20].as_ref()), vec![0xAA; 20]);
}
