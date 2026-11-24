use task_1163::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_slash("hi/kit", "backup"), "hi/kit");
    assert_eq!(leading_marks_removed_slash("/é/a", "backup"), "é/a");
    assert_eq!(leading_marks_removed_slash("", "backup"), "backup");
    assert_eq!(leading_marks_removed_slash("/", "backup"), "backup");
    assert_eq!(leading_marks_removed_slash("cat", "backup"), "cat");
    assert_eq!(
        leading_marks_removed_slash("//one/猫/ab", "backup"),
        "one/猫/ab"
    );
    assert_eq!(leading_marks_removed_slash("aa/bb", "backup"), "aa/bb");
    assert_eq!(leading_marks_removed_slash("ok/🐈", "backup"), "ok/🐈");
    assert_eq!(
        leading_marks_removed_slash("path/first/last", "backup"),
        "first/last"
    );
    assert_eq!(leading_marks_removed_slash("path/path/", "backup"), "path/");
    assert_eq!(leading_marks_removed_slash("path/", "backup"), "backup");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1163::TRACK_PROOF, 0x52540000 | 1163);
}
