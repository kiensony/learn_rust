use task_1549::*;

#[test]
fn edge_cases() {
    assert_eq!(packet_gray_code_frequent_readings(&[4], 0), vec![]);
    assert_eq!(
        packet_gray_code_frequent_readings(&[9, 2, 2, 8, 0, 5], 1),
        vec![3u32]
    );
    assert_eq!(
        packet_gray_code_frequent_readings(&[0, 0, 0], 2),
        vec![0u32]
    );
    assert_eq!(
        packet_gray_code_frequent_readings(&[255, 1, 255, 2], 3),
        vec![128u32, 1u32, 3u32]
    );
    assert_eq!(
        packet_gray_code_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 256),
        vec![
            2u32, 5u32, 13u32, 10u32, 31u32, 8u32, 0u32, 23u32, 12u32, 16u32
        ]
    );
    assert_eq!(
        packet_gray_code_frequent_readings(&[17, 17, 1, 0, 2], 0),
        vec![]
    );
    assert_eq!(
        packet_gray_code_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 1),
        vec![2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1549::TRACK_PROOF, 0x52540000 | 1549);
}
