use task_542::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_equals("hi=kit"), 6);
    assert_eq!(marks_removed_at_both_ends_equals("=é=a"), 3);
    assert_eq!(marks_removed_at_both_ends_equals(""), 0);
    assert_eq!(marks_removed_at_both_ends_equals("="), 0);
    assert_eq!(marks_removed_at_both_ends_equals("cat"), 3);
    assert_eq!(marks_removed_at_both_ends_equals("==one=猫=ab"), 8);
    assert_eq!(marks_removed_at_both_ends_equals("aa=bb"), 5);
    assert_eq!(marks_removed_at_both_ends_equals("ok=🐈"), 4);
    assert_eq!(marks_removed_at_both_ends_equals("aa=bb;meta=123"), 5);
    assert_eq!(marks_removed_at_both_ends_equals(";ignore=all"), 0);
    assert_eq!(marks_removed_at_both_ends_equals("a==b;"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_542::TRACK_PROOF, 0x52540000 | 542);
}
