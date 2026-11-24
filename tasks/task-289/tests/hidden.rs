use task_289::*;

#[test]
fn edge_cases() {
    assert_eq!(wrap_a_message_plus(""), "++");
    assert_eq!(wrap_a_message_plus("+"), "+++");
    assert_eq!(wrap_a_message_plus("++"), "++++");
    assert_eq!(wrap_a_message_plus(" café "), "+ café +");
    assert_eq!(wrap_a_message_plus("a+b"), "+a+b+");
    assert_eq!(wrap_a_message_plus("🐈+猫"), "+🐈+猫+");
}

#[test]
fn submission_marker() {
    assert_eq!(task_289::TRACK_PROOF, 0x52540000 | 289);
}
