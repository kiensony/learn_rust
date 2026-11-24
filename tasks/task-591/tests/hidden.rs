use task_591::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_equals("hi=kit"), "hi=kit");
    assert_eq!(marks_removed_at_both_ends_equals("=é=a"), "é=a");
    assert_eq!(marks_removed_at_both_ends_equals(""), "");
    assert_eq!(marks_removed_at_both_ends_equals("="), "");
    assert_eq!(marks_removed_at_both_ends_equals("cat"), "cat");
    assert_eq!(
        marks_removed_at_both_ends_equals("==one=猫=ab"),
        "one=猫=ab"
    );
    assert_eq!(marks_removed_at_both_ends_equals("aa=bb"), "aa=bb");
    assert_eq!(marks_removed_at_both_ends_equals("ok=🐈"), "ok=🐈");
    assert_eq!(marks_removed_at_both_ends_equals("aa=bb;meta=123"), "aa=bb");
    assert_eq!(marks_removed_at_both_ends_equals(";ignore=all"), "");
    assert_eq!(marks_removed_at_both_ends_equals("a==b;"), "a==b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_591::TRACK_PROOF, 0x52540000 | 591);
}
