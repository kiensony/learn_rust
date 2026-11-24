use task_541::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_equals("hi=kit"), 6);
    assert_eq!(closing_marks_removed_equals("=é=a"), 4);
    assert_eq!(closing_marks_removed_equals(""), 0);
    assert_eq!(closing_marks_removed_equals("="), 0);
    assert_eq!(closing_marks_removed_equals("cat"), 3);
    assert_eq!(closing_marks_removed_equals("==one=猫=ab"), 10);
    assert_eq!(closing_marks_removed_equals("aa=bb"), 5);
    assert_eq!(closing_marks_removed_equals("ok=🐈"), 4);
    assert_eq!(closing_marks_removed_equals("aa=bb;meta=123"), 5);
    assert_eq!(closing_marks_removed_equals(";ignore=all"), 0);
    assert_eq!(closing_marks_removed_equals("a==b;"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_541::TRACK_PROOF, 0x52540000 | 541);
}
