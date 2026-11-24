use task_286::*;

#[test]
fn edge_cases() {
    assert_eq!(blank_notice_tilde(""), "~");
    assert_eq!(blank_notice_tilde("~"), "~");
    assert_eq!(blank_notice_tilde("~~"), "~~");
    assert_eq!(blank_notice_tilde(" café "), "café");
    assert_eq!(blank_notice_tilde("a~b"), "a~b");
    assert_eq!(blank_notice_tilde("🐈~猫"), "🐈~猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_286::TRACK_PROOF, 0x52540000 | 286);
}
