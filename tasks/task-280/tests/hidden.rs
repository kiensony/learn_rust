use task_280::*;

#[test]
fn edge_cases() {
    assert_eq!(remove_a_prefix_tilde(""), "");
    assert_eq!(remove_a_prefix_tilde("~"), "");
    assert_eq!(remove_a_prefix_tilde("~~"), "~");
    assert_eq!(remove_a_prefix_tilde(" café "), " café ");
    assert_eq!(remove_a_prefix_tilde("a~b"), "a~b");
    assert_eq!(remove_a_prefix_tilde("🐈~猫"), "🐈~猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_280::TRACK_PROOF, 0x52540000 | 280);
}
