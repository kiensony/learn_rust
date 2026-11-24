use task_574::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_slash("hi/kit"), "hi");
    assert_eq!(first_nonempty_field_slash("/é/a"), "é");
    assert_eq!(first_nonempty_field_slash(""), "");
    assert_eq!(first_nonempty_field_slash("/"), "");
    assert_eq!(first_nonempty_field_slash("cat"), "cat");
    assert_eq!(first_nonempty_field_slash("//one/猫/ab"), "one");
    assert_eq!(first_nonempty_field_slash("aa/bb"), "aa");
    assert_eq!(first_nonempty_field_slash("ok/🐈"), "ok");
    assert_eq!(first_nonempty_field_slash("path/first/last"), "first");
    assert_eq!(first_nonempty_field_slash("path/path/"), "path");
    assert_eq!(first_nonempty_field_slash("path/"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_574::TRACK_PROOF, 0x52540000 | 574);
}
