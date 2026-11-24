use task_523::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_slash("hi/kit"), 3);
    assert_eq!(remainder_after_first_mark_slash("/é/a"), 3);
    assert_eq!(remainder_after_first_mark_slash(""), 0);
    assert_eq!(remainder_after_first_mark_slash("/"), 0);
    assert_eq!(remainder_after_first_mark_slash("cat"), 0);
    assert_eq!(remainder_after_first_mark_slash("//one/猫/ab"), 9);
    assert_eq!(remainder_after_first_mark_slash("aa/bb"), 2);
    assert_eq!(remainder_after_first_mark_slash("ok/🐈"), 1);
    assert_eq!(remainder_after_first_mark_slash("path/first/last"), 4);
    assert_eq!(remainder_after_first_mark_slash("path/path/"), 0);
    assert_eq!(remainder_after_first_mark_slash("path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_523::TRACK_PROOF, 0x52540000 | 523);
}
