use task_1538::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packet_gray_code_steps(&[9, 2, 2, 8, 0, 5]),
        vec![10u32, 0u32, 9u32, 12u32, 7u32]
    );
    assert_eq!(packet_gray_code_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        packet_gray_code_steps(&[255, 1, 255, 2]),
        vec![127u32, 127u32, 125u32]
    );
    assert_eq!(
        packet_gray_code_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 8u32, 3u32, 21u32, 23u32, 8u32, 23u32, 11u32, 4u32]
    );
    assert_eq!(
        packet_gray_code_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 24u32, 1u32, 3u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1538::TRACK_PROOF, 0x52540000 | 1538);
}
