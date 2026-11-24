use task_2310::*;

#[test]
fn edge_cases() {
    assert_eq!(
        warmup_discard_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 0u32, 8u32, 2u32]
    );
    assert_eq!(warmup_discard_reverse(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        warmup_discard_reverse(&[255, 1, 255, 2]),
        vec![2u32, 255u32]
    );
    assert_eq!(
        warmup_discard_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![31u32, 8u32, 26u32, 0u32, 15u32, 21u32, 12u32, 9u32]
    );
    assert_eq!(
        warmup_discard_reverse(&[17, 17, 1, 0, 2]),
        vec![2u32, 0u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2310::TRACK_PROOF, 0x52540000 | 2310);
}
