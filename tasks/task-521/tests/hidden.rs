use task_521::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_slash("hi/kit"), 6);
    assert_eq!(closing_marks_removed_slash("/é/a"), 4);
    assert_eq!(closing_marks_removed_slash(""), 0);
    assert_eq!(closing_marks_removed_slash("/"), 0);
    assert_eq!(closing_marks_removed_slash("cat"), 3);
    assert_eq!(closing_marks_removed_slash("//one/猫/ab"), 10);
    assert_eq!(closing_marks_removed_slash("aa/bb"), 5);
    assert_eq!(closing_marks_removed_slash("ok/🐈"), 4);
    assert_eq!(closing_marks_removed_slash("path/first/last"), 10);
    assert_eq!(closing_marks_removed_slash("path/path/"), 4);
    assert_eq!(closing_marks_removed_slash("path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_521::TRACK_PROOF, 0x52540000 | 521);
}
