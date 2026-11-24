use task_571::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_slash("hi/kit"), "hi/kit");
    assert_eq!(marks_removed_at_both_ends_slash("/é/a"), "é/a");
    assert_eq!(marks_removed_at_both_ends_slash(""), "");
    assert_eq!(marks_removed_at_both_ends_slash("/"), "");
    assert_eq!(marks_removed_at_both_ends_slash("cat"), "cat");
    assert_eq!(marks_removed_at_both_ends_slash("//one/猫/ab"), "one/猫/ab");
    assert_eq!(marks_removed_at_both_ends_slash("aa/bb"), "aa/bb");
    assert_eq!(marks_removed_at_both_ends_slash("ok/🐈"), "ok/🐈");
    assert_eq!(
        marks_removed_at_both_ends_slash("path/first/last"),
        "first/last"
    );
    assert_eq!(marks_removed_at_both_ends_slash("path/path/"), "path");
    assert_eq!(marks_removed_at_both_ends_slash("path/"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_571::TRACK_PROOF, 0x52540000 | 571);
}
