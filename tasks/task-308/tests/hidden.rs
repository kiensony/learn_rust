use task_308::*;

#[test]
fn edge_cases() {
    assert_eq!(normalize_spaces_bang(""), "");
    assert_eq!(normalize_spaces_bang("!"), "!");
    assert_eq!(normalize_spaces_bang("!!"), "!!");
    assert_eq!(normalize_spaces_bang(" café "), "café");
    assert_eq!(normalize_spaces_bang("a!b"), "a!b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_308::TRACK_PROOF, 0x52540000 | 308);
}
