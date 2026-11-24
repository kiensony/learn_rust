use task_737::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_equals(Some("hi=kit")), None);
    assert_eq!(closing_marks_removed_equals(Some("=é=a")), None);
    assert_eq!(closing_marks_removed_equals(Some("")), None);
    assert_eq!(closing_marks_removed_equals(Some("=")), None);
    assert_eq!(closing_marks_removed_equals(Some("cat")), None);
    assert_eq!(closing_marks_removed_equals(Some("==one=猫=ab")), None);
    assert_eq!(closing_marks_removed_equals(Some("aa=bb")), None);
    assert_eq!(closing_marks_removed_equals(Some("ok=🐈")), None);
    assert_eq!(closing_marks_removed_equals(Some("aa=bb;meta=123")), None);
    assert_eq!(closing_marks_removed_equals(Some(";ignore=all")), None);
    assert_eq!(closing_marks_removed_equals(Some("a==b;")), None);
    assert_eq!(closing_marks_removed_equals(None), None);
    assert_eq!(closing_marks_removed_equals(Some("3=72")), None);
    assert_eq!(closing_marks_removed_equals(Some("=9")), None);
    assert_eq!(closing_marks_removed_equals(Some("3300")), Some(3300));
    assert_eq!(closing_marks_removed_equals(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_737::TRACK_PROOF, 0x52540000 | 737);
}
