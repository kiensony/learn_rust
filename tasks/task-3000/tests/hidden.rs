use task_3000::*;

#[test]
fn edge_cases() {
    assert_eq!(affected_ticks(&[(8, 9), (0, 2), (2, 8), (1, 3)]), 9);
    assert_eq!(affected_ticks(&[(3, 3), (9, 1)]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_3000::TRACK_PROOF, 0x52540000 | 3000);
}
