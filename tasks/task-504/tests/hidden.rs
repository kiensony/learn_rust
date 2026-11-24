use task_504::*;

#[test]
fn edge_cases() {
    assert_eq!(columns(&[]), (vec![], vec![]));
    assert_eq!(columns(&[(0, 0), (0, 0)]), (vec![0, 0], vec![0, 0]));
}

#[test]
fn submission_marker() {
    assert_eq!(task_504::TRACK_PROOF, 0x52540000 | 504);
}
