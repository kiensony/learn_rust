use task_2457::*;

#[test]
fn edge_cases() {
    assert_eq!(checked_copy(&[], 0, 0), Some(vec![]));
    assert_eq!(checked_copy(&[4], 1, 0), Some(vec![]));
    assert_eq!(checked_copy(&[4], usize::MAX, 2), None);
    assert_eq!(checked_copy(&[0], 2, 0), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2457::TRACK_PROOF, 0x52540000 | 2457);
}
