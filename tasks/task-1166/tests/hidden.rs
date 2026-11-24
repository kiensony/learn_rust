use task_1166::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_slash("hi/kit", "backup"), "kit");
    assert_eq!(remainder_after_first_mark_slash("/é/a", "backup"), "é/a");
    assert_eq!(remainder_after_first_mark_slash("", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_slash("/", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_slash("cat", "backup"), "backup");
    assert_eq!(
        remainder_after_first_mark_slash("//one/猫/ab", "backup"),
        "/one/猫/ab"
    );
    assert_eq!(remainder_after_first_mark_slash("aa/bb", "backup"), "bb");
    assert_eq!(remainder_after_first_mark_slash("ok/🐈", "backup"), "🐈");
    assert_eq!(
        remainder_after_first_mark_slash("path/first/last", "backup"),
        "last"
    );
    assert_eq!(
        remainder_after_first_mark_slash("path/path/", "backup"),
        "backup"
    );
    assert_eq!(
        remainder_after_first_mark_slash("path/", "backup"),
        "backup"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1166::TRACK_PROOF, 0x52540000 | 1166);
}
