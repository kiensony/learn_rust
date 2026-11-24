use task_1940::*;

#[test]
fn edge_cases() {
    assert_eq!(
        locked_batch_totals_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 10u32, 11u32]
    );
    assert_eq!(locked_batch_totals_reverse(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        locked_batch_totals_reverse(&[255, 1, 255, 2]),
        vec![257u32, 256u32]
    );
    assert_eq!(
        locked_batch_totals_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![39u32, 26u32, 36u32, 21u32, 9u32]
    );
    assert_eq!(
        locked_batch_totals_reverse(&[17, 17, 1, 0, 2]),
        vec![2u32, 1u32, 34u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1940::TRACK_PROOF, 0x52540000 | 1940);
}
