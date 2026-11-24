use task_268::*;

#[test]
fn edge_cases() {
    assert_eq!(normalize_spaces_hash(""), "");
    assert_eq!(normalize_spaces_hash("#"), "#");
    assert_eq!(normalize_spaces_hash("##"), "##");
    assert_eq!(normalize_spaces_hash(" café "), "café");
    assert_eq!(normalize_spaces_hash("a#b"), "a#b");
    assert_eq!(normalize_spaces_hash("🐈#猫"), "🐈#猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_268::TRACK_PROOF, 0x52540000 | 268);
}
