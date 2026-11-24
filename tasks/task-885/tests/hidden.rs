use task_885::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_equals("hi=kit"), 8);
    assert_eq!(leading_marks_removed_equals("=é=a"), 6);
    assert_eq!(leading_marks_removed_equals(""), 2);
    assert_eq!(leading_marks_removed_equals("="), 2);
    assert_eq!(leading_marks_removed_equals("cat"), 5);
    assert_eq!(leading_marks_removed_equals("==one=猫=ab"), 12);
    assert_eq!(leading_marks_removed_equals("aa=bb"), 7);
    assert_eq!(leading_marks_removed_equals("ok=🐈"), 9);
    assert_eq!(leading_marks_removed_equals("aa=bb;meta=123"), 7);
    assert_eq!(leading_marks_removed_equals(";ignore=all"), 2);
    assert_eq!(leading_marks_removed_equals("a==b;"), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_885::TRACK_PROOF, 0x52540000 | 885);
}
