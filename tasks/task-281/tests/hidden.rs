use task_281::*;

#[test]
fn edge_cases() {
    assert_eq!(remove_a_suffix_tilde(""), "");
    assert_eq!(remove_a_suffix_tilde("~"), "");
    assert_eq!(remove_a_suffix_tilde("~~"), "~");
    assert_eq!(remove_a_suffix_tilde(" café "), " café ");
    assert_eq!(remove_a_suffix_tilde("a~b"), "a~b");
    assert_eq!(remove_a_suffix_tilde("🐈~猫"), "🐈~猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_281::TRACK_PROOF, 0x52540000 | 281);
}
