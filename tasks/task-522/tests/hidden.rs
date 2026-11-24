use task_522::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_slash("hi/kit"), 6);
    assert_eq!(marks_removed_at_both_ends_slash("/é/a"), 3);
    assert_eq!(marks_removed_at_both_ends_slash(""), 0);
    assert_eq!(marks_removed_at_both_ends_slash("/"), 0);
    assert_eq!(marks_removed_at_both_ends_slash("cat"), 3);
    assert_eq!(marks_removed_at_both_ends_slash("//one/猫/ab"), 8);
    assert_eq!(marks_removed_at_both_ends_slash("aa/bb"), 5);
    assert_eq!(marks_removed_at_both_ends_slash("ok/🐈"), 4);
    assert_eq!(marks_removed_at_both_ends_slash("path/first/last"), 10);
    assert_eq!(marks_removed_at_both_ends_slash("path/path/"), 4);
    assert_eq!(marks_removed_at_both_ends_slash("path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_522::TRACK_PROOF, 0x52540000 | 522);
}
