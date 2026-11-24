use task_490::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_equals(("hi=kit", "x")), 6);
    assert_eq!(leading_marks_removed_equals(("=é=a", "z=w")), 4);
    assert_eq!(leading_marks_removed_equals(("", "x")), 1);
    assert_eq!(leading_marks_removed_equals(("=", "z=w")), 3);
    assert_eq!(leading_marks_removed_equals(("cat", "x")), 3);
    assert_eq!(leading_marks_removed_equals(("==one=猫=ab", "z=w")), 10);
    assert_eq!(leading_marks_removed_equals(("aa=bb", "x")), 5);
    assert_eq!(leading_marks_removed_equals(("ok=🐈", "z=w")), 7);
    assert_eq!(leading_marks_removed_equals(("aa=bb;meta=123", "x")), 5);
    assert_eq!(leading_marks_removed_equals((";ignore=all", "z=w")), 0);
    assert_eq!(leading_marks_removed_equals(("a==b;", "x")), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_490::TRACK_PROOF, 0x52540000 | 490);
}
