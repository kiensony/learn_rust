use task_1785::*;

#[test]
fn edge_cases() {
    assert_eq!(
        token_multiplier_changes(&[9, 2, 2, 8, 0, 5]),
        vec![36u32, 8u32, 32u32, 0u32, 20u32]
    );
    assert_eq!(token_multiplier_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        token_multiplier_changes(&[255, 1, 255, 2]),
        vec![1020u32, 4u32, 1020u32, 8u32]
    );
    assert_eq!(
        token_multiplier_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            12u32, 24u32, 36u32, 48u32, 84u32, 60u32, 0u32, 104u32, 32u32, 124u32
        ]
    );
    assert_eq!(
        token_multiplier_changes(&[17, 17, 1, 0, 2]),
        vec![68u32, 4u32, 0u32, 8u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1785::TRACK_PROOF, 0x52540000 | 1785);
}
