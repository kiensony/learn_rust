use task_1147::*;

#[test]
fn edge_cases() {
    assert_eq!(apply_all(-9, &[]), -9);
    assert_eq!(apply_all(32767, &[&Offset(1)]), 32767);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1147::TRACK_PROOF, 0x52540000 | 1147);
}
