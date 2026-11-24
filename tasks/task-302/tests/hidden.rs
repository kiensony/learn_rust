use task_302::*;

#[test]
fn edge_cases() {
    assert_eq!(boundary_trim_bang(""), "");
    assert_eq!(boundary_trim_bang("!"), "");
    assert_eq!(boundary_trim_bang("!!"), "");
    assert_eq!(boundary_trim_bang(" café "), " café ");
    assert_eq!(boundary_trim_bang("a!b"), "a!b");
    assert_eq!(boundary_trim_bang("🐈!猫"), "🐈!猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_302::TRACK_PROOF, 0x52540000 | 302);
}
