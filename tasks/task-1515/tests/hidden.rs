use task_1515::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packet_low_nibble_changes(&[9, 2, 2, 8, 0, 5]),
        vec![9u32, 2u32, 8u32, 0u32, 5u32]
    );
    assert_eq!(packet_low_nibble_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        packet_low_nibble_changes(&[255, 1, 255, 2]),
        vec![15u32, 1u32, 15u32, 2u32]
    );
    assert_eq!(
        packet_low_nibble_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            3u32, 6u32, 9u32, 12u32, 5u32, 15u32, 0u32, 10u32, 8u32, 15u32
        ]
    );
    assert_eq!(
        packet_low_nibble_changes(&[17, 17, 1, 0, 2]),
        vec![1u32, 0u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1515::TRACK_PROOF, 0x52540000 | 1515);
}
