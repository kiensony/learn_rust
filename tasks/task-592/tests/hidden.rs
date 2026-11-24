use task_592::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_equals("hi=kit"), "kit");
    assert_eq!(remainder_after_first_mark_equals("=é=a"), "é=a");
    assert_eq!(remainder_after_first_mark_equals(""), "");
    assert_eq!(remainder_after_first_mark_equals("="), "");
    assert_eq!(remainder_after_first_mark_equals("cat"), "");
    assert_eq!(
        remainder_after_first_mark_equals("==one=猫=ab"),
        "=one=猫=ab"
    );
    assert_eq!(remainder_after_first_mark_equals("aa=bb"), "bb");
    assert_eq!(remainder_after_first_mark_equals("ok=🐈"), "🐈");
    assert_eq!(remainder_after_first_mark_equals("aa=bb;meta=123"), "bb");
    assert_eq!(remainder_after_first_mark_equals(";ignore=all"), "");
    assert_eq!(remainder_after_first_mark_equals("a==b;"), "=b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_592::TRACK_PROOF, 0x52540000 | 592);
}
