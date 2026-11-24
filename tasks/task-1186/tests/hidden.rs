use task_1186::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_equals("hi=kit", "backup"), "kit");
    assert_eq!(remainder_after_first_mark_equals("=é=a", "backup"), "é=a");
    assert_eq!(remainder_after_first_mark_equals("", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_equals("=", "backup"), "backup");
    assert_eq!(remainder_after_first_mark_equals("cat", "backup"), "backup");
    assert_eq!(
        remainder_after_first_mark_equals("==one=猫=ab", "backup"),
        "=one=猫=ab"
    );
    assert_eq!(remainder_after_first_mark_equals("aa=bb", "backup"), "bb");
    assert_eq!(remainder_after_first_mark_equals("ok=🐈", "backup"), "🐈");
    assert_eq!(
        remainder_after_first_mark_equals("aa=bb;meta=123", "backup"),
        "bb"
    );
    assert_eq!(
        remainder_after_first_mark_equals(";ignore=all", "backup"),
        "backup"
    );
    assert_eq!(remainder_after_first_mark_equals("a==b;", "backup"), "=b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1186::TRACK_PROOF, 0x52540000 | 1186);
}
