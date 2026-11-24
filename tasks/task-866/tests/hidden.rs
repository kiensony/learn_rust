use task_866::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_slash("hi/kit"), 8);
    assert_eq!(closing_marks_removed_slash("/é/a"), 7);
    assert_eq!(closing_marks_removed_slash(""), 2);
    assert_eq!(closing_marks_removed_slash("/"), 2);
    assert_eq!(closing_marks_removed_slash("cat"), 5);
    assert_eq!(closing_marks_removed_slash("//one/猫/ab"), 14);
    assert_eq!(closing_marks_removed_slash("aa/bb"), 7);
    assert_eq!(closing_marks_removed_slash("ok/🐈"), 9);
    assert_eq!(closing_marks_removed_slash("path/first/last"), 12);
    assert_eq!(closing_marks_removed_slash("path/path/"), 6);
    assert_eq!(closing_marks_removed_slash("path/"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_866::TRACK_PROOF, 0x52540000 | 866);
}
