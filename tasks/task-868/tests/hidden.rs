use task_868::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_slash("hi/kit"), 5);
    assert_eq!(remainder_after_first_mark_slash("/é/a"), 6);
    assert_eq!(remainder_after_first_mark_slash(""), 2);
    assert_eq!(remainder_after_first_mark_slash("/"), 2);
    assert_eq!(remainder_after_first_mark_slash("cat"), 2);
    assert_eq!(remainder_after_first_mark_slash("//one/猫/ab"), 13);
    assert_eq!(remainder_after_first_mark_slash("aa/bb"), 4);
    assert_eq!(remainder_after_first_mark_slash("ok/🐈"), 6);
    assert_eq!(remainder_after_first_mark_slash("path/first/last"), 6);
    assert_eq!(remainder_after_first_mark_slash("path/path/"), 2);
    assert_eq!(remainder_after_first_mark_slash("path/"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_868::TRACK_PROOF, 0x52540000 | 868);
}
