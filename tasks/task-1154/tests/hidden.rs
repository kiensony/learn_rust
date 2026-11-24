use task_1154::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon("", "backup"), "backup");
    assert_eq!(closing_marks_removed_colon(":", "backup"), "backup");
    assert_eq!(closing_marks_removed_colon("cat", "backup"), "cat");
    assert_eq!(
        closing_marks_removed_colon("::one:猫:ab", "backup"),
        "::one:猫:ab"
    );
    assert_eq!(closing_marks_removed_colon("aa:bb", "backup"), "aa:bb");
    assert_eq!(closing_marks_removed_colon("ok:🐈", "backup"), "ok:🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1154::TRACK_PROOF, 0x52540000 | 1154);
}
