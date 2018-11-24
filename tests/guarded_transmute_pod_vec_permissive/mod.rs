#![cfg(feature = "std")]
use self::super::LeToNative;
use safe_transmute::guarded_transmute_pod_vec_permissive;

#[test]
fn too_short() {
    assert_eq!(guarded_transmute_pod_vec_permissive::<u16>(vec![]), Ok(vec![]));
    assert_eq!(guarded_transmute_pod_vec_permissive::<u16>(vec![0x00]), Ok(vec![]));
}

#[test]
fn just_enough() {
    assert_eq!(
        guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01].le_to_native::<u16>()),
        Ok(vec![0x0100u16])
    );
    assert_eq!(
        guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02].le_to_native::<u16>()),
        Ok(vec![0x0100u16, 0x0200u16])
    );
}

#[test]
fn too_much() {
    assert_eq!(
        guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01, 0x00].le_to_native::<u16>()),
        Ok(vec![0x0100u16])
    );
    assert_eq!(
        guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02, 0x00].le_to_native::<u16>()),
        Ok(vec![0x0100u16, 0x0200u16])
    );
    assert_eq!(
        guarded_transmute_pod_vec_permissive::<u16>(vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03, 0x00].le_to_native::<u16>()),
        Ok(vec![0x0100u16, 0x0200u16, 0x0300u16])
    );
}
