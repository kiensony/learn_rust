use task_562::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon(""), "");
    assert_eq!(marks_removed_at_both_ends_colon(":"), "");
    assert_eq!(marks_removed_at_both_ends_colon("cat"), "cat");
    assert_eq!(marks_removed_at_both_ends_colon("::one:猫:ab"), "one:猫:ab");
    assert_eq!(marks_removed_at_both_ends_colon("aa:bb"), "aa:bb");
    assert_eq!(marks_removed_at_both_ends_colon("ok:🐈"), "ok:🐈");
}

#[test]
fn submission_marker() {
    assert_eq!(task_562::TRACK_PROOF, 0x52540000 | 562);
}
