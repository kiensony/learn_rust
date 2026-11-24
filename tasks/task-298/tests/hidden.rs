use task_298::*;

#[test]
fn edge_cases() {
    assert_eq!(normalize_spaces_plus(""), "");
    assert_eq!(normalize_spaces_plus("+"), "+");
    assert_eq!(normalize_spaces_plus("++"), "++");
    assert_eq!(normalize_spaces_plus(" café "), "café");
    assert_eq!(normalize_spaces_plus("a+b"), "a+b");
    assert_eq!(normalize_spaces_plus("🐈+猫"), "🐈+猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_298::TRACK_PROOF, 0x52540000 | 298);
}
