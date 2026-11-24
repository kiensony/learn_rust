use task_1237::*;

#[test]
fn edge_cases() {
    assert_eq!(
        seek_offsets_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![2u32, 5u32, 8u32]
    );
    assert_eq!(seek_offsets_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(seek_offsets_distinct(&[255, 1, 255, 2]), vec![1u32, 2u32]);
    assert_eq!(
        seek_offsets_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![6u32, 12u32, 15u32, 26u32, 31u32]
    );
    assert_eq!(seek_offsets_distinct(&[17, 17, 1, 0, 2]), vec![0u32, 17u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1237::TRACK_PROOF, 0x52540000 | 1237);
}
