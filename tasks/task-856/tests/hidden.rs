use task_856::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(""), 2);
    assert_eq!(closing_marks_removed_colon(":"), 2);
    assert_eq!(closing_marks_removed_colon("cat"), 5);
    assert_eq!(closing_marks_removed_colon("::one:猫:ab"), 14);
    assert_eq!(closing_marks_removed_colon("aa:bb"), 7);
    assert_eq!(closing_marks_removed_colon("ok:🐈"), 9);
}

#[test]
fn submission_marker() {
    assert_eq!(task_856::TRACK_PROOF, 0x52540000 | 856);
}
