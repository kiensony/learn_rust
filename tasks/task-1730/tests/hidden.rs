use task_1730::*;

#[test]
fn edge_cases() {
    assert_eq!(
        boolean_macro_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 1u32, 0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(boolean_macro_reverse(&[0, 0, 0]), vec![1u32, 1u32, 1u32]);
    assert_eq!(
        boolean_macro_reverse(&[255, 1, 255, 2]),
        vec![0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(
        boolean_macro_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 0u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(
        boolean_macro_reverse(&[17, 17, 1, 0, 2]),
        vec![0u32, 1u32, 0u32, 0u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1730::TRACK_PROOF, 0x52540000 | 1730);
}
