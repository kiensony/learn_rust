use task_288::*;

#[test]
fn edge_cases() {
    assert_eq!(normalize_spaces_tilde(""), "");
    assert_eq!(normalize_spaces_tilde("~"), "~");
    assert_eq!(normalize_spaces_tilde("~~"), "~~");
    assert_eq!(normalize_spaces_tilde(" café "), "café");
    assert_eq!(normalize_spaces_tilde("a~b"), "a~b");
    assert_eq!(normalize_spaces_tilde("🐈~猫"), "🐈~猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_288::TRACK_PROOF, 0x52540000 | 288);
}
