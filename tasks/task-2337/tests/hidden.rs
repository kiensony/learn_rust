use task_2337::*;

#[test]
fn edge_cases() {
    assert_eq!(
        batch_variation_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 7u32]
    );
    assert_eq!(batch_variation_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(batch_variation_distinct(&[255, 1, 255, 2]), vec![254u32]);
    assert_eq!(
        batch_variation_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![9u32, 23u32, 26u32]
    );
    assert_eq!(
        batch_variation_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 17u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2337::TRACK_PROOF, 0x52540000 | 2337);
}
