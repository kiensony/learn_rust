use task_543::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_equals("hi=kit"), 3);
    assert_eq!(remainder_after_first_mark_equals("=é=a"), 3);
    assert_eq!(remainder_after_first_mark_equals(""), 0);
    assert_eq!(remainder_after_first_mark_equals("="), 0);
    assert_eq!(remainder_after_first_mark_equals("cat"), 0);
    assert_eq!(remainder_after_first_mark_equals("==one=猫=ab"), 9);
    assert_eq!(remainder_after_first_mark_equals("aa=bb"), 2);
    assert_eq!(remainder_after_first_mark_equals("ok=🐈"), 1);
    assert_eq!(remainder_after_first_mark_equals("aa=bb;meta=123"), 2);
    assert_eq!(remainder_after_first_mark_equals(";ignore=all"), 0);
    assert_eq!(remainder_after_first_mark_equals("a==b;"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_543::TRACK_PROOF, 0x52540000 | 543);
}
