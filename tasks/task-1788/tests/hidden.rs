use task_1788::*;

#[test]
fn edge_cases() {
    assert_eq!(
        token_multiplier_steps(&[9, 2, 2, 8, 0, 5]),
        vec![28u32, 0u32, 24u32, 32u32, 20u32]
    );
    assert_eq!(token_multiplier_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        token_multiplier_steps(&[255, 1, 255, 2]),
        vec![1016u32, 1016u32, 1012u32]
    );
    assert_eq!(
        token_multiplier_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            12u32, 12u32, 12u32, 36u32, 24u32, 60u32, 104u32, 72u32, 92u32
        ]
    );
    assert_eq!(
        token_multiplier_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 64u32, 4u32, 8u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1788::TRACK_PROOF, 0x52540000 | 1788);
}
