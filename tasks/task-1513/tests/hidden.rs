use task_1513::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packet_low_nibble_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((0u32, 9u32))
    );
    assert_eq!(packet_low_nibble_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        packet_low_nibble_bounds(&[255, 1, 255, 2]),
        Some((1u32, 15u32))
    );
    assert_eq!(
        packet_low_nibble_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((0u32, 15u32))
    );
    assert_eq!(
        packet_low_nibble_bounds(&[17, 17, 1, 0, 2]),
        Some((0u32, 2u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1513::TRACK_PROOF, 0x52540000 | 1513);
}
