use task_1028::*;

#[test]
fn edge_cases() {
    assert_eq!(slots(&[]), 0);
    assert_eq!(slots(&[2, 2, 2]), 3);
    assert_eq!(slots(&[0, 0, 0, 0, 0, 0]), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1028::TRACK_PROOF, 0x52540000 | 1028);
}
