use task_266::*;

#[test]
fn edge_cases() {
    assert_eq!(blank_notice_hash(""), "#");
    assert_eq!(blank_notice_hash("#"), "#");
    assert_eq!(blank_notice_hash("##"), "##");
    assert_eq!(blank_notice_hash(" café "), "café");
    assert_eq!(blank_notice_hash("a#b"), "a#b");
    assert_eq!(blank_notice_hash("🐈#猫"), "🐈#猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_266::TRACK_PROOF, 0x52540000 | 266);
}
