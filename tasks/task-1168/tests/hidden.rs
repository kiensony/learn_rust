use task_1168::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_slash("hi/kit", "backup"), "hi");
    assert_eq!(first_nonempty_field_slash("/é/a", "backup"), "é");
    assert_eq!(first_nonempty_field_slash("", "backup"), "backup");
    assert_eq!(first_nonempty_field_slash("/", "backup"), "backup");
    assert_eq!(first_nonempty_field_slash("cat", "backup"), "cat");
    assert_eq!(first_nonempty_field_slash("//one/猫/ab", "backup"), "one");
    assert_eq!(first_nonempty_field_slash("aa/bb", "backup"), "aa");
    assert_eq!(first_nonempty_field_slash("ok/🐈", "backup"), "ok");
    assert_eq!(
        first_nonempty_field_slash("path/first/last", "backup"),
        "first"
    );
    assert_eq!(first_nonempty_field_slash("path/path/", "backup"), "path");
    assert_eq!(first_nonempty_field_slash("path/", "backup"), "backup");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1168::TRACK_PROOF, 0x52540000 | 1168);
}
