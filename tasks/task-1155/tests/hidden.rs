use task_1155::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon("", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_colon(":", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_colon("cat", "backup"), "cat");
    assert_eq!(
        marks_removed_at_both_ends_colon("::one:猫:ab", "backup"),
        "one:猫:ab"
    );
    assert_eq!(marks_removed_at_both_ends_colon("aa:bb", "backup"), "aa:bb");
    assert_eq!(marks_removed_at_both_ends_colon("ok:🐈", "backup"), "ok:🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1155::TRACK_PROOF, 0x52540000 | 1155);
}
