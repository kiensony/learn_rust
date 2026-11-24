use task_763::*;

#[test]
fn edge_cases() {
    assert_eq!(served(&[], 9), vec![]);
    assert_eq!(served(&[9], 0), vec![]);
    assert_eq!(served(&[7, 7, 1], 2), vec![7, 7]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_763::TRACK_PROOF, 0x52540000 | 763);
}
