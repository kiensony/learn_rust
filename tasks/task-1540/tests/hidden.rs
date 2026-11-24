use task_1540::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packet_gray_code_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![7u32, 0u32, 12u32, 3u32, 3u32, 13u32]
    );
    assert_eq!(packet_gray_code_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        packet_gray_code_reverse(&[255, 1, 255, 2]),
        vec![3u32, 128u32, 1u32, 128u32]
    );
    assert_eq!(
        packet_gray_code_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            16u32, 12u32, 23u32, 0u32, 8u32, 31u32, 10u32, 13u32, 5u32, 2u32
        ]
    );
    assert_eq!(
        packet_gray_code_reverse(&[17, 17, 1, 0, 2]),
        vec![3u32, 0u32, 1u32, 25u32, 25u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1540::TRACK_PROOF, 0x52540000 | 1540);
}
