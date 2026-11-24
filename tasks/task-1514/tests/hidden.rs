use task_1514::*;

#[test]
fn edge_cases() {
    assert_eq!(packet_low_nibble_peaks(&[9, 2, 2, 8, 0, 5]), vec![9u32]);
    assert_eq!(packet_low_nibble_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(packet_low_nibble_peaks(&[255, 1, 255, 2]), vec![15u32]);
    assert_eq!(
        packet_low_nibble_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 6u32, 9u32, 12u32, 15u32]
    );
    assert_eq!(
        packet_low_nibble_peaks(&[17, 17, 1, 0, 2]),
        vec![1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1514::TRACK_PROOF, 0x52540000 | 1514);
}
