use task_1156::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_colon("", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_colon(":", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_colon("cat", "backup"), "backup");
    assert_eq!(
        remainder_after_first_mark_colon("::one:猫:ab", "backup"),
        ":one:猫:ab"
    );
    assert_eq!(remainder_after_first_mark_colon("aa:bb", "backup"), "bb");
    assert_eq!(remainder_after_first_mark_colon("ok:🐈", "backup"), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1156::TRACK_PROOF, 0x52540000 | 1156);
}
