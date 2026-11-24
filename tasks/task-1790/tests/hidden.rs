use task_1790::*;

#[test]
fn edge_cases() {
    assert_eq!(
        token_multiplier_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![20u32, 0u32, 32u32, 8u32, 8u32, 36u32]
    );
    assert_eq!(token_multiplier_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        token_multiplier_reverse(&[255, 1, 255, 2]),
        vec![8u32, 1020u32, 4u32, 1020u32]
    );
    assert_eq!(
        token_multiplier_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            124u32, 32u32, 104u32, 0u32, 60u32, 84u32, 48u32, 36u32, 24u32, 12u32
        ]
    );
    assert_eq!(
        token_multiplier_reverse(&[17, 17, 1, 0, 2]),
        vec![8u32, 0u32, 4u32, 68u32, 68u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1790::TRACK_PROOF, 0x52540000 | 1790);
}
