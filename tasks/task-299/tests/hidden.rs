use task_299::*;

#[test]
fn edge_cases() {
    assert_eq!(wrap_a_message_bang(""), "!!");
    assert_eq!(wrap_a_message_bang("!"), "!!!");
    assert_eq!(wrap_a_message_bang("!!"), "!!!!");
    assert_eq!(wrap_a_message_bang(" café "), "! café !");
    assert_eq!(wrap_a_message_bang("a!b"), "!a!b!");
    assert_eq!(wrap_a_message_bang("🐈!猫"), "!🐈!猫!");
}

#[test]
fn submission_marker() {
    assert_eq!(task_299::TRACK_PROOF, 0x52540000 | 299);
}
