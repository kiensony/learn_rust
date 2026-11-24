use task_2328::*;

#[test]
fn edge_cases() {
    assert_eq!(batch_medians_steps(&[9, 2, 2, 8, 0, 5]), vec![3u32]);
    assert_eq!(batch_medians_steps(&[0, 0, 0]), vec![]);
    assert_eq!(batch_medians_steps(&[255, 1, 255, 2]), vec![253u32]);
    assert_eq!(
        batch_medians_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![9u32, 7u32, 23u32]
    );
    assert_eq!(batch_medians_steps(&[17, 17, 1, 0, 2]), vec![17u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2328::TRACK_PROOF, 0x52540000 | 2328);
}
