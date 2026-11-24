use task_1518::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packet_low_nibble_steps(&[9, 2, 2, 8, 0, 5]),
        vec![7u32, 0u32, 6u32, 8u32, 5u32]
    );
    assert_eq!(packet_low_nibble_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        packet_low_nibble_steps(&[255, 1, 255, 2]),
        vec![14u32, 14u32, 13u32]
    );
    assert_eq!(
        packet_low_nibble_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 3u32, 3u32, 7u32, 10u32, 15u32, 10u32, 2u32, 7u32]
    );
    assert_eq!(
        packet_low_nibble_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 0u32, 1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1518::TRACK_PROOF, 0x52540000 | 1518);
}
