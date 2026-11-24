use task_2331::*;

#[test]
fn edge_cases() {
    assert_eq!(batch_variation_trace(&[9, 2, 2, 8, 0, 5]), vec![7u32, 5u32]);
    assert_eq!(batch_variation_trace(&[0, 0, 0]), vec![0u32]);
    assert_eq!(batch_variation_trace(&[255, 1, 255, 2]), vec![254u32]);
    assert_eq!(
        batch_variation_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![9u32, 26u32, 23u32]
    );
    assert_eq!(batch_variation_trace(&[17, 17, 1, 0, 2]), vec![17u32, 0u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2331::TRACK_PROOF, 0x52540000 | 2331);
}
