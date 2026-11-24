use task_276::*;

#[test]
fn edge_cases() {
    assert_eq!(blank_notice_star(""), "*");
    assert_eq!(blank_notice_star("*"), "*");
    assert_eq!(blank_notice_star("**"), "**");
    assert_eq!(blank_notice_star(" café "), "café");
    assert_eq!(blank_notice_star("a*b"), "a*b");
    assert_eq!(blank_notice_star("🐈*猫"), "🐈*猫");
}

#[test]
fn submission_marker() {
    assert_eq!(task_276::TRACK_PROOF, 0x52540000 | 276);
}
