use task_290::*;

#[test]
fn edge_cases() {
    assert_eq!(remove_a_prefix_plus(""), "");
    assert_eq!(remove_a_prefix_plus("+"), "");
    assert_eq!(remove_a_prefix_plus("++"), "+");
    assert_eq!(remove_a_prefix_plus(" café "), " café ");
    assert_eq!(remove_a_prefix_plus("a+b"), "a+b");
    assert_eq!(remove_a_prefix_plus("🐈+猫"), "🐈+猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_290::TRACK_PROOF, 0x52540000 | 290);
}
