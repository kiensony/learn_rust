use task_2308::*;

#[test]
fn edge_cases() {
    assert_eq!(
        warmup_discard_steps(&[9, 2, 2, 8, 0, 5]),
        vec![6u32, 8u32, 5u32]
    );
    assert_eq!(warmup_discard_steps(&[0, 0, 0]), vec![]);
    assert_eq!(warmup_discard_steps(&[255, 1, 255, 2]), vec![253u32]);
    assert_eq!(
        warmup_discard_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 9u32, 6u32, 15u32, 26u32, 18u32, 23u32]
    );
    assert_eq!(warmup_discard_steps(&[17, 17, 1, 0, 2]), vec![1u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2308::TRACK_PROOF, 0x52540000 | 2308);
}
