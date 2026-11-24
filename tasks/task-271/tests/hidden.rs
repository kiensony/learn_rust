use task_271::*;

#[test]
fn edge_cases() {
    assert_eq!(remove_a_suffix_star(""), "");
    assert_eq!(remove_a_suffix_star("*"), "");
    assert_eq!(remove_a_suffix_star("**"), "*");
    assert_eq!(remove_a_suffix_star(" café "), " café ");
    assert_eq!(remove_a_suffix_star("a*b"), "a*b");
    assert_eq!(remove_a_suffix_star("🐈*猫"), "🐈*猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_271::TRACK_PROOF, 0x52540000 | 271);
}
