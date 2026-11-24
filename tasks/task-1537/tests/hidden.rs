use task_1537::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packet_gray_code_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 3u32, 7u32, 12u32, 13u32]
    );
    assert_eq!(packet_gray_code_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        packet_gray_code_distinct(&[255, 1, 255, 2]),
        vec![1u32, 3u32, 128u32]
    );
    assert_eq!(
        packet_gray_code_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            0u32, 2u32, 5u32, 8u32, 10u32, 12u32, 13u32, 16u32, 23u32, 31u32
        ]
    );
    assert_eq!(
        packet_gray_code_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 1u32, 3u32, 25u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1537::TRACK_PROOF, 0x52540000 | 1537);
}
