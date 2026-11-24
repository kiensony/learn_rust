use task_1423::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remaining_budget_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((0u32, 103u32))
    );
    assert_eq!(remaining_budget_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        remaining_budget_bounds(&[255, 1, 255, 2]),
        Some((0u32, 4285u32))
    );
    assert_eq!(
        remaining_budget_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((0u32, 477u32))
    );
    assert_eq!(
        remaining_budget_bounds(&[17, 17, 1, 0, 2]),
        Some((0u32, 239u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1423::TRACK_PROOF, 0x52540000 | 1423);
}
