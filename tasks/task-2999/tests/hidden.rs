use task_2999::*;

#[test]
fn edge_cases() {
    assert_eq!(handoff(vec![], 3), (vec![], vec![]));
    assert_eq!(handoff(vec![1, 2], usize::MAX), (vec![1, 2], vec![]));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2999::TRACK_PROOF, 0x52540000 | 2999);
}
