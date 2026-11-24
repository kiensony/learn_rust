use task_511::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(""), 0);
    assert_eq!(closing_marks_removed_colon(":"), 0);
    assert_eq!(closing_marks_removed_colon("cat"), 3);
    assert_eq!(closing_marks_removed_colon("::one:猫:ab"), 10);
    assert_eq!(closing_marks_removed_colon("aa:bb"), 5);
    assert_eq!(closing_marks_removed_colon("ok:🐈"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_511::TRACK_PROOF, 0x52540000 | 511);
}
