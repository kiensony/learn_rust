use task_1744::*;

#[test]
fn edge_cases() {
    assert_eq!(rotate_macro_peaks(&[9, 2, 2, 8, 0, 5]), vec![36u32]);
    assert_eq!(rotate_macro_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(rotate_macro_peaks(&[255, 1, 255, 2]), vec![255u32]);
    assert_eq!(
        rotate_macro_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![12u32, 24u32, 36u32, 48u32, 84u32, 104u32, 124u32]
    );
    assert_eq!(rotate_macro_peaks(&[17, 17, 1, 0, 2]), vec![68u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1744::TRACK_PROOF, 0x52540000 | 1744);
}
