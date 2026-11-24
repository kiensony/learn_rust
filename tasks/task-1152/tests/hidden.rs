use task_1152::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon("", "backup"), "backup");
    assert_eq!(closing_field_colon(":", "backup"), "backup");
    assert_eq!(closing_field_colon("cat", "backup"), "cat");
    assert_eq!(closing_field_colon("::one:猫:ab", "backup"), "ab");
    assert_eq!(closing_field_colon("aa:bb", "backup"), "bb");
    assert_eq!(closing_field_colon("ok:🐈", "backup"), "🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1152::TRACK_PROOF, 0x52540000 | 1152);
}
