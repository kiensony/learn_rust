use task_1424::*;

#[test]
fn edge_cases() {
    assert_eq!(remaining_budget_peaks(&[9, 2, 2, 8, 0, 5]), vec![103u32]);
    assert_eq!(remaining_budget_peaks(&[0, 0, 0]), vec![0u32]);
    assert_eq!(remaining_budget_peaks(&[255, 1, 255, 2]), vec![4285u32]);
    assert_eq!(
        remaining_budget_peaks(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![1u32, 52u32, 103u32, 154u32, 307u32, 392u32, 477u32]
    );
    assert_eq!(remaining_budget_peaks(&[17, 17, 1, 0, 2]), vec![239u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1424::TRACK_PROOF, 0x52540000 | 1424);
}
