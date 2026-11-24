use task_1158::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon("", "backup"), "backup");
    assert_eq!(first_nonempty_field_colon(":", "backup"), "backup");
    assert_eq!(first_nonempty_field_colon("cat", "backup"), "cat");
    assert_eq!(first_nonempty_field_colon("::one:猫:ab", "backup"), "one");
    assert_eq!(first_nonempty_field_colon("aa:bb", "backup"), "aa");
    assert_eq!(first_nonempty_field_colon("ok:🐈", "backup"), "ok");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1158::TRACK_PROOF, 0x52540000 | 1158);
}
