use task_1185::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_equals("hi=kit", "backup"),
        "hi=kit"
    );
    assert_eq!(marks_removed_at_both_ends_equals("=é=a", "backup"), "é=a");
    assert_eq!(marks_removed_at_both_ends_equals("", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_equals("=", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_equals("cat", "backup"), "cat");
    assert_eq!(
        marks_removed_at_both_ends_equals("==one=猫=ab", "backup"),
        "one=猫=ab"
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("aa=bb", "backup"),
        "aa=bb"
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("ok=🐈", "backup"),
        "ok=🐈"
    );
    assert_eq!(
        marks_removed_at_both_ends_equals("aa=bb;meta=123", "backup"),
        "aa=bb"
    );
    assert_eq!(
        marks_removed_at_both_ends_equals(";ignore=all", "backup"),
        "backup"
    );
    assert_eq!(marks_removed_at_both_ends_equals("a==b;", "backup"), "a==b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1185::TRACK_PROOF, 0x52540000 | 1185);
}
