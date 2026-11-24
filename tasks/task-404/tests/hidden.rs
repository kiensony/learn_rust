use task_404::*;

#[test]
fn edge_cases() {
    assert_eq!(has_header(&[], &[]), true);
    assert_eq!(has_header(&[], &[0]), false);
    assert_eq!(has_header(&[1], &[1, 2]), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_404::TRACK_PROOF, 0x52540000 | 404);
}
