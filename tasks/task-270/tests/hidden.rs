use task_270::*;

#[test]
fn edge_cases() {
    assert_eq!(remove_a_prefix_star(""), "");
    assert_eq!(remove_a_prefix_star("*"), "");
    assert_eq!(remove_a_prefix_star("**"), "*");
    assert_eq!(remove_a_prefix_star(" café "), " café ");
    assert_eq!(remove_a_prefix_star("a*b"), "a*b");
    assert_eq!(remove_a_prefix_star("🐈*猫"), "🐈*猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_270::TRACK_PROOF, 0x52540000 | 270);
}
