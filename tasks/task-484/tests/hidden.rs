use task_484::*;

#[test]
fn edge_cases() {
    assert_eq!(window(&[], 0, 0), Some(&[][..]));
    assert_eq!(window(&[7], 2, 0), None);
    assert_eq!(window(&[7], 1, usize::MAX), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_484::TRACK_PROOF, 0x52540000 | 484);
}
