use task_2175::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_comparison_budget_changes(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 1u32, 2u32, 3u32, 4u32, 5u32]
    );
    assert_eq!(
        prefix_comparison_budget_changes(&[0, 0, 0]),
        vec![0u32, 1u32, 2u32]
    );
    assert_eq!(
        prefix_comparison_budget_changes(&[255, 1, 255, 2]),
        vec![0u32, 1u32, 2u32, 3u32]
    );
    assert_eq!(
        prefix_comparison_budget_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32]
    );
    assert_eq!(
        prefix_comparison_budget_changes(&[17, 17, 1, 0, 2]),
        vec![0u32, 1u32, 2u32, 3u32, 4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2175::TRACK_PROOF, 0x52540000 | 2175);
}
