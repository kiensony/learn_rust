use task_570::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_slash("hi/kit"), "hi/kit");
    assert_eq!(closing_marks_removed_slash("/é/a"), "/é/a");
    assert_eq!(closing_marks_removed_slash(""), "");
    assert_eq!(closing_marks_removed_slash("/"), "");
    assert_eq!(closing_marks_removed_slash("cat"), "cat");
    assert_eq!(closing_marks_removed_slash("//one/猫/ab"), "//one/猫/ab");
    assert_eq!(closing_marks_removed_slash("aa/bb"), "aa/bb");
    assert_eq!(closing_marks_removed_slash("ok/🐈"), "ok/🐈");
    assert_eq!(closing_marks_removed_slash("path/first/last"), "first/last");
    assert_eq!(closing_marks_removed_slash("path/path/"), "path");
    assert_eq!(closing_marks_removed_slash("path/"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_570::TRACK_PROOF, 0x52540000 | 570);
}
