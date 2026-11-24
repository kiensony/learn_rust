use task_1085::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_equals(&"hi=kit"), 6);
    assert_eq!(marks_removed_at_both_ends_equals(&String::from("=é=a")), 4);
    assert_eq!(marks_removed_at_both_ends_equals(&""), 0);
    assert_eq!(marks_removed_at_both_ends_equals(&String::from("=")), 0);
    assert_eq!(marks_removed_at_both_ends_equals(&"cat"), 3);
    assert_eq!(
        marks_removed_at_both_ends_equals(&String::from("==one=猫=ab")),
        10
    );
    assert_eq!(marks_removed_at_both_ends_equals(&"aa=bb"), 5);
    assert_eq!(marks_removed_at_both_ends_equals(&String::from("ok=🐈")), 7);
    assert_eq!(marks_removed_at_both_ends_equals(&"aa=bb;meta=123"), 5);
    assert_eq!(
        marks_removed_at_both_ends_equals(&String::from(";ignore=all")),
        0
    );
    assert_eq!(marks_removed_at_both_ends_equals(&"a==b;"), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1085::TRACK_PROOF, 0x52540000 | 1085);
}
