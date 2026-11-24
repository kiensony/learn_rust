use task_279::*;

#[test]
fn edge_cases() {
    assert_eq!(wrap_a_message_tilde(""), "~~");
    assert_eq!(wrap_a_message_tilde("~"), "~~~");
    assert_eq!(wrap_a_message_tilde("~~"), "~~~~");
    assert_eq!(wrap_a_message_tilde(" café "), "~ café ~");
    assert_eq!(wrap_a_message_tilde("a~b"), "~a~b~");
    assert_eq!(wrap_a_message_tilde("🐈~猫"), "~🐈~猫~");
}

#[test]
fn submission_marker() {
    assert_eq!(task_279::TRACK_PROOF, 0x52540000 | 279);
}
