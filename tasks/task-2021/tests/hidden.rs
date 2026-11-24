use task_2021::*;

#[test]
fn edge_cases() {
    assert_eq!(
        polled_groups_trace(&[9, 2, 2, 8, 0, 5]),
        vec![2u32, 0u32, 0u32, 2u32, 0u32, 1u32]
    );
    assert_eq!(polled_groups_trace(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        polled_groups_trace(&[255, 1, 255, 2]),
        vec![63u32, 0u32, 63u32, 0u32]
    );
    assert_eq!(
        polled_groups_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 1u32, 2u32, 3u32, 5u32, 3u32, 0u32, 6u32, 2u32, 7u32]
    );
    assert_eq!(
        polled_groups_trace(&[17, 17, 1, 0, 2]),
        vec![4u32, 4u32, 0u32, 0u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2021::TRACK_PROOF, 0x52540000 | 2021);
}
