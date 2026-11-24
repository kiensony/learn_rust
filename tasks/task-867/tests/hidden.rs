use task_867::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_slash("hi/kit"), 8);
    assert_eq!(marks_removed_at_both_ends_slash("/é/a"), 6);
    assert_eq!(marks_removed_at_both_ends_slash(""), 2);
    assert_eq!(marks_removed_at_both_ends_slash("/"), 2);
    assert_eq!(marks_removed_at_both_ends_slash("cat"), 5);
    assert_eq!(marks_removed_at_both_ends_slash("//one/猫/ab"), 12);
    assert_eq!(marks_removed_at_both_ends_slash("aa/bb"), 7);
    assert_eq!(marks_removed_at_both_ends_slash("ok/🐈"), 9);
    assert_eq!(marks_removed_at_both_ends_slash("path/first/last"), 12);
    assert_eq!(marks_removed_at_both_ends_slash("path/path/"), 6);
    assert_eq!(marks_removed_at_both_ends_slash("path/"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_867::TRACK_PROOF, 0x52540000 | 867);
}
