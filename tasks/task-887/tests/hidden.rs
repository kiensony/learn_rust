use task_887::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_equals("hi=kit"), 8);
    assert_eq!(marks_removed_at_both_ends_equals("=é=a"), 6);
    assert_eq!(marks_removed_at_both_ends_equals(""), 2);
    assert_eq!(marks_removed_at_both_ends_equals("="), 2);
    assert_eq!(marks_removed_at_both_ends_equals("cat"), 5);
    assert_eq!(marks_removed_at_both_ends_equals("==one=猫=ab"), 12);
    assert_eq!(marks_removed_at_both_ends_equals("aa=bb"), 7);
    assert_eq!(marks_removed_at_both_ends_equals("ok=🐈"), 9);
    assert_eq!(marks_removed_at_both_ends_equals("aa=bb;meta=123"), 7);
    assert_eq!(marks_removed_at_both_ends_equals(";ignore=all"), 2);
    assert_eq!(marks_removed_at_both_ends_equals("a==b;"), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_887::TRACK_PROOF, 0x52540000 | 887);
}
