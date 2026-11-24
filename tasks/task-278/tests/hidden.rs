use task_278::*;

#[test]
fn edge_cases() {
    assert_eq!(normalize_spaces_star(""), "");
    assert_eq!(normalize_spaces_star("*"), "*");
    assert_eq!(normalize_spaces_star("**"), "**");
    assert_eq!(normalize_spaces_star(" café "), "café");
    assert_eq!(normalize_spaces_star("a*b"), "a*b");
    assert_eq!(normalize_spaces_star("🐈*猫"), "🐈*猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_278::TRACK_PROOF, 0x52540000 | 278);
}
