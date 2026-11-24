use task_492::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_equals(("hi=kit", "x")), 6);
    assert_eq!(marks_removed_at_both_ends_equals(("=é=a", "z=w")), 4);
    assert_eq!(marks_removed_at_both_ends_equals(("", "x")), 1);
    assert_eq!(marks_removed_at_both_ends_equals(("=", "z=w")), 3);
    assert_eq!(marks_removed_at_both_ends_equals(("cat", "x")), 3);
    assert_eq!(
        marks_removed_at_both_ends_equals(("==one=猫=ab", "z=w")),
        10
    );
    assert_eq!(marks_removed_at_both_ends_equals(("aa=bb", "x")), 5);
    assert_eq!(marks_removed_at_both_ends_equals(("ok=🐈", "z=w")), 7);
    assert_eq!(
        marks_removed_at_both_ends_equals(("aa=bb;meta=123", "x")),
        5
    );
    assert_eq!(marks_removed_at_both_ends_equals((";ignore=all", "z=w")), 0);
    assert_eq!(marks_removed_at_both_ends_equals(("a==b;", "x")), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_492::TRACK_PROOF, 0x52540000 | 492);
}
