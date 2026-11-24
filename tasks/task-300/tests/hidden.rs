use task_300::*;

#[test]
fn edge_cases() {
    assert_eq!(remove_a_prefix_bang(""), "");
    assert_eq!(remove_a_prefix_bang("!"), "");
    assert_eq!(remove_a_prefix_bang("!!"), "!");
    assert_eq!(remove_a_prefix_bang(" café "), " café ");
    assert_eq!(remove_a_prefix_bang("a!b"), "a!b");
    assert_eq!(remove_a_prefix_bang("🐈!猫"), "🐈!猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_300::TRACK_PROOF, 0x52540000 | 300);
}
