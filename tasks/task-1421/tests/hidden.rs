use task_1421::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remaining_budget_trace(&[9, 2, 2, 8, 0, 5]),
        vec![103u32, 0u32, 0u32, 86u32, 0u32, 35u32]
    );
    assert_eq!(remaining_budget_trace(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        remaining_budget_trace(&[255, 1, 255, 2]),
        vec![4285u32, 0u32, 4285u32, 0u32]
    );
    assert_eq!(
        remaining_budget_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            1u32, 52u32, 103u32, 154u32, 307u32, 205u32, 0u32, 392u32, 86u32, 477u32
        ]
    );
    assert_eq!(
        remaining_budget_trace(&[17, 17, 1, 0, 2]),
        vec![239u32, 239u32, 0u32, 0u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1421::TRACK_PROOF, 0x52540000 | 1421);
}
