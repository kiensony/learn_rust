use task_236::*;

#[test]
fn edge_cases() {
    assert_eq!(open_count(&[]), 0);
    assert_eq!(open_count(&[false, false]), 0);
    assert_eq!(open_count(&[true, true, true]), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_236::TRACK_PROOF, 0x52540000 | 236);
}
