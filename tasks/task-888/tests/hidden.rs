use task_888::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_equals("hi=kit"), 5);
    assert_eq!(remainder_after_first_mark_equals("=é=a"), 6);
    assert_eq!(remainder_after_first_mark_equals(""), 2);
    assert_eq!(remainder_after_first_mark_equals("="), 2);
    assert_eq!(remainder_after_first_mark_equals("cat"), 2);
    assert_eq!(remainder_after_first_mark_equals("==one=猫=ab"), 13);
    assert_eq!(remainder_after_first_mark_equals("aa=bb"), 4);
    assert_eq!(remainder_after_first_mark_equals("ok=🐈"), 6);
    assert_eq!(remainder_after_first_mark_equals("aa=bb;meta=123"), 4);
    assert_eq!(remainder_after_first_mark_equals(";ignore=all"), 2);
    assert_eq!(remainder_after_first_mark_equals("a==b;"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_888::TRACK_PROOF, 0x52540000 | 888);
}
