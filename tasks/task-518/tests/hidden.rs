use task_518::*;

#[test]
fn edge_cases() {
    assert_eq!(scalar_at("", 0), None);
    assert_eq!(scalar_at("a", 1), None);
    assert_eq!(scalar_at("e\u{301}", 1), Some('\u{301}'));
}

#[test]
fn submission_marker() {
    assert_eq!(task_518::TRACK_PROOF, 0x52540000 | 518);
}
