use task_2523::*;

#[test]
fn edge_cases() {
    assert_eq!(
        abi_signed_decode_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((0u32, 9u32))
    );
    assert_eq!(abi_signed_decode_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        abi_signed_decode_bounds(&[255, 1, 255, 2]),
        Some((1u32, 2u32))
    );
    assert_eq!(
        abi_signed_decode_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((0u32, 31u32))
    );
    assert_eq!(
        abi_signed_decode_bounds(&[17, 17, 1, 0, 2]),
        Some((0u32, 17u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2523::TRACK_PROOF, 0x52540000 | 2523);
}
