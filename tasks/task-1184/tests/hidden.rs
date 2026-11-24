use task_1184::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_equals("hi=kit", "backup"), "hi=kit");
    assert_eq!(closing_marks_removed_equals("=é=a", "backup"), "=é=a");
    assert_eq!(closing_marks_removed_equals("", "backup"), "backup");
    assert_eq!(closing_marks_removed_equals("=", "backup"), "backup");
    assert_eq!(closing_marks_removed_equals("cat", "backup"), "cat");
    assert_eq!(
        closing_marks_removed_equals("==one=猫=ab", "backup"),
        "==one=猫=ab"
    );
    assert_eq!(closing_marks_removed_equals("aa=bb", "backup"), "aa=bb");
    assert_eq!(closing_marks_removed_equals("ok=🐈", "backup"), "ok=🐈");
    assert_eq!(
        closing_marks_removed_equals("aa=bb;meta=123", "backup"),
        "aa=bb"
    );
    assert_eq!(
        closing_marks_removed_equals(";ignore=all", "backup"),
        "backup"
    );
    assert_eq!(closing_marks_removed_equals("a==b;", "backup"), "a==b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1184::TRACK_PROOF, 0x52540000 | 1184);
}
