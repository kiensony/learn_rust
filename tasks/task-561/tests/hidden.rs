use task_561::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(""), "");
    assert_eq!(closing_marks_removed_colon(":"), "");
    assert_eq!(closing_marks_removed_colon("cat"), "cat");
    assert_eq!(closing_marks_removed_colon("::one:猫:ab"), "::one:猫:ab");
    assert_eq!(closing_marks_removed_colon("aa:bb"), "aa:bb");
    assert_eq!(closing_marks_removed_colon("ok:🐈"), "ok:🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_561::TRACK_PROOF, 0x52540000 | 561);
}
