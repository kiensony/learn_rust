use task_306::*;

#[test]
fn edge_cases() {
    assert_eq!(blank_notice_bang(""), "!");
    assert_eq!(blank_notice_bang("!"), "!");
    assert_eq!(blank_notice_bang("!!"), "!!");
    assert_eq!(blank_notice_bang(" café "), "café");
    assert_eq!(blank_notice_bang("a!b"), "a!b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_306::TRACK_PROOF, 0x52540000 | 306);
}
