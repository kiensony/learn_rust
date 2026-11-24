use task_307::*;

#[test]
fn edge_cases() {
    assert_eq!(count_separators_bang(""), "0");
    assert_eq!(count_separators_bang("!"), "1");
    assert_eq!(count_separators_bang("!!"), "2");
    assert_eq!(count_separators_bang(" café "), "0");
    assert_eq!(count_separators_bang("a!b"), "1");
}

#[test]
fn submission_marker() {
    assert_eq!(task_307::TRACK_PROOF, 0x52540000 | 307);
}
