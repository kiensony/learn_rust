use task_1387::*;

#[test]
fn edge_cases() {
    assert_eq!(
        first_frame_byte_distinct(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        vec![0u32, 4u32, 7u32, 9u32]
    );
    assert_eq!(first_frame_byte_distinct(&[3, 1]), vec![]);
    assert_eq!(
        first_frame_byte_distinct(&[1, 0, 0, 1, 255]),
        vec![0u32, 255u32]
    );
    assert_eq!(
        first_frame_byte_distinct(&[2, 3, 1, 2, 3, 1, 1, 2]),
        vec![2u32, 3u32]
    );
    assert_eq!(first_frame_byte_distinct(&[255, 0]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1387::TRACK_PROOF, 0x52540000 | 1387);
}
