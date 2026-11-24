use task_493::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_equals(("hi=kit", "x")), 3);
    assert_eq!(remainder_after_first_mark_equals(("=é=a", "z=w")), 4);
    assert_eq!(remainder_after_first_mark_equals(("", "x")), 0);
    assert_eq!(remainder_after_first_mark_equals(("=", "z=w")), 1);
    assert_eq!(remainder_after_first_mark_equals(("cat", "x")), 0);
    assert_eq!(
        remainder_after_first_mark_equals(("==one=猫=ab", "z=w")),
        11
    );
    assert_eq!(remainder_after_first_mark_equals(("aa=bb", "x")), 2);
    assert_eq!(remainder_after_first_mark_equals(("ok=🐈", "z=w")), 4);
    assert_eq!(
        remainder_after_first_mark_equals(("aa=bb;meta=123", "x")),
        2
    );
    assert_eq!(remainder_after_first_mark_equals((";ignore=all", "z=w")), 0);
    assert_eq!(remainder_after_first_mark_equals(("a==b;", "x")), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_493::TRACK_PROOF, 0x52540000 | 493);
}
