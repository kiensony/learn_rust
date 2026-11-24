use task_461::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(("", "x")), 1);
    assert_eq!(closing_marks_removed_colon((":", "z:w")), 3);
    assert_eq!(closing_marks_removed_colon(("cat", "x")), 3);
    assert_eq!(closing_marks_removed_colon(("::one:猫:ab", "z:w")), 12);
    assert_eq!(closing_marks_removed_colon(("aa:bb", "x")), 5);
    assert_eq!(closing_marks_removed_colon(("ok:🐈", "z:w")), 7);
}

#[test]
fn submission_marker() {
    assert_eq!(task_461::TRACK_PROOF, 0x52540000 | 461);
}
