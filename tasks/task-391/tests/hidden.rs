use task_391::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_equals(&String::from("hi=kit")), 6);
    assert_eq!(closing_marks_removed_equals(&String::from("=é=a")), 5);
    assert_eq!(closing_marks_removed_equals(&String::from("")), 0);
    assert_eq!(closing_marks_removed_equals(&String::from("=")), 0);
    assert_eq!(closing_marks_removed_equals(&String::from("cat")), 3);
    assert_eq!(
        closing_marks_removed_equals(&String::from("==one=猫=ab")),
        12
    );
    assert_eq!(closing_marks_removed_equals(&String::from("aa=bb")), 5);
    assert_eq!(closing_marks_removed_equals(&String::from("ok=🐈")), 7);
    assert_eq!(
        closing_marks_removed_equals(&String::from("aa=bb;meta=123")),
        5
    );
    assert_eq!(
        closing_marks_removed_equals(&String::from(";ignore=all")),
        0
    );
    assert_eq!(closing_marks_removed_equals(&String::from("a==b;")), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_391::TRACK_PROOF, 0x52540000 | 391);
}
