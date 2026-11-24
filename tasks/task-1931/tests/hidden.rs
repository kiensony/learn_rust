use task_1931::*;

#[test]
fn edge_cases() {
    assert_eq!(
        locked_batch_totals_trace(&[9, 2, 2, 8, 0, 5]),
        vec![11u32, 10u32, 5u32]
    );
    assert_eq!(locked_batch_totals_trace(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        locked_batch_totals_trace(&[255, 1, 255, 2]),
        vec![256u32, 257u32]
    );
    assert_eq!(
        locked_batch_totals_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![9u32, 21u32, 36u32, 26u32, 39u32]
    );
    assert_eq!(
        locked_batch_totals_trace(&[17, 17, 1, 0, 2]),
        vec![34u32, 1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1931::TRACK_PROOF, 0x52540000 | 1931);
}
