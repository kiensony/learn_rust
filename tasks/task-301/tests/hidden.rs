use task_301::*;

#[test]
fn edge_cases() {
    assert_eq!(remove_a_suffix_bang(""), "");
    assert_eq!(remove_a_suffix_bang("!"), "");
    assert_eq!(remove_a_suffix_bang("!!"), "!");
    assert_eq!(remove_a_suffix_bang(" café "), " café ");
    assert_eq!(remove_a_suffix_bang("a!b"), "a!b");
    assert_eq!(remove_a_suffix_bang("🐈!猫"), "🐈!猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_301::TRACK_PROOF, 0x52540000 | 301);
}
