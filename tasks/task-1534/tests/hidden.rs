use task_1534::*;

#[test]
fn edge_cases() {
    assert_eq!(packet_gray_code_peaks(&[9, 2, 2, 8, 0, 5]), vec![13u32]);
    assert_eq!(packet_gray_code_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(packet_gray_code_peaks(&[255, 1, 255, 2]), vec![128u32]);
    assert_eq!(
        packet_gray_code_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![2u32, 5u32, 13u32, 31u32]
    );
    assert_eq!(packet_gray_code_peaks(&[17, 17, 1, 0, 2]), vec![25u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1534::TRACK_PROOF, 0x52540000 | 1534);
}
