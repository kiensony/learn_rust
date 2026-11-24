use task_998::*;

#[test]
fn edge_cases() {
    assert_eq!(runs(vec![]), vec![]);
    assert_eq!(runs(vec![0, 0, 0]), vec![(0, 3)]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_998::TRACK_PROOF, 0x52540000 | 998);
}
