use task_1165::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_slash("hi/kit", "backup"),
        "hi/kit"
    );
    assert_eq!(marks_removed_at_both_ends_slash("/é/a", "backup"), "é/a");
    assert_eq!(marks_removed_at_both_ends_slash("", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_slash("/", "backup"), "backup");
    assert_eq!(marks_removed_at_both_ends_slash("cat", "backup"), "cat");
    assert_eq!(
        marks_removed_at_both_ends_slash("//one/猫/ab", "backup"),
        "one/猫/ab"
    );
    assert_eq!(marks_removed_at_both_ends_slash("aa/bb", "backup"), "aa/bb");
    assert_eq!(marks_removed_at_both_ends_slash("ok/🐈", "backup"), "ok/🐈");
    assert_eq!(
        marks_removed_at_both_ends_slash("path/first/last", "backup"),
        "first/last"
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("path/path/", "backup"),
        "path"
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("path/", "backup"),
        "backup"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1165::TRACK_PROOF, 0x52540000 | 1165);
}
