use task_305::*;

#[test]
fn edge_cases() {
    assert_eq!(needs_update("", "  "), false);
    assert_eq!(needs_update("a b", "ab"), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_305::TRACK_PROOF, 0x52540000 | 305);
}
