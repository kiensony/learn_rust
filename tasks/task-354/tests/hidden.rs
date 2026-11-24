use task_354::*;

#[test]
fn edge_cases() {
    assert_eq!(closed_visits(0), 0);
    assert_eq!(closed_visits(1), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_354::TRACK_PROOF, 0x52540000 | 354);
}
