use task_296::*;

#[test]
fn edge_cases() {
    assert_eq!(blank_notice_plus(""), "+");
    assert_eq!(blank_notice_plus("+"), "+");
    assert_eq!(blank_notice_plus("++"), "++");
    assert_eq!(blank_notice_plus(" café "), "café");
    assert_eq!(blank_notice_plus("a+b"), "a+b");
    assert_eq!(blank_notice_plus("🐈+猫"), "🐈+猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_296::TRACK_PROOF, 0x52540000 | 296);
}
