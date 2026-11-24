use task_256::*;

#[test]
fn edge_cases() {
    assert_eq!(steps_to(0), 0);
    assert_eq!(steps_to(1), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_256::TRACK_PROOF, 0x52540000 | 256);
}
