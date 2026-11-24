use task_572::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_slash("hi/kit"), "kit");
    assert_eq!(remainder_after_first_mark_slash("/é/a"), "é/a");
    assert_eq!(remainder_after_first_mark_slash(""), "");
    assert_eq!(remainder_after_first_mark_slash("/"), "");
    assert_eq!(remainder_after_first_mark_slash("cat"), "");
    assert_eq!(
        remainder_after_first_mark_slash("//one/猫/ab"),
        "/one/猫/ab"
    );
    assert_eq!(remainder_after_first_mark_slash("aa/bb"), "bb");
    assert_eq!(remainder_after_first_mark_slash("ok/🐈"), "🐈");
    assert_eq!(remainder_after_first_mark_slash("path/first/last"), "last");
    assert_eq!(remainder_after_first_mark_slash("path/path/"), "");
    assert_eq!(remainder_after_first_mark_slash("path/"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_572::TRACK_PROOF, 0x52540000 | 572);
}
