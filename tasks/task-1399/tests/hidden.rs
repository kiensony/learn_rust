use task_1399::*;

#[test]
fn edge_cases() {
    assert_eq!(first_frame_byte_frequent_readings(&[0], 0), vec![]);
    assert_eq!(
        first_frame_byte_frequent_readings(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 1),
        vec![7u32]
    );
    assert_eq!(first_frame_byte_frequent_readings(&[3, 1], 2), vec![]);
    assert_eq!(
        first_frame_byte_frequent_readings(&[1, 0, 0, 1, 255], 3),
        vec![0u32, 255u32]
    );
    assert_eq!(
        first_frame_byte_frequent_readings(&[2, 3, 1, 2, 3, 1, 1, 2], 256),
        vec![3u32, 2u32]
    );
    assert_eq!(first_frame_byte_frequent_readings(&[255, 0], 0), vec![]);
    assert_eq!(
        first_frame_byte_frequent_readings(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2], 1),
        vec![7u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1399::TRACK_PROOF, 0x52540000 | 1399);
}
