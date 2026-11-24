use task_1088::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_equals(&"hi=kit"), 2);
    assert_eq!(first_nonempty_field_equals(&String::from("=é=a")), 2);
    assert_eq!(first_nonempty_field_equals(&""), 0);
    assert_eq!(first_nonempty_field_equals(&String::from("=")), 0);
    assert_eq!(first_nonempty_field_equals(&"cat"), 3);
    assert_eq!(first_nonempty_field_equals(&String::from("==one=猫=ab")), 3);
    assert_eq!(first_nonempty_field_equals(&"aa=bb"), 2);
    assert_eq!(first_nonempty_field_equals(&String::from("ok=🐈")), 2);
    assert_eq!(first_nonempty_field_equals(&"aa=bb;meta=123"), 2);
    assert_eq!(first_nonempty_field_equals(&String::from(";ignore=all")), 0);
    assert_eq!(first_nonempty_field_equals(&"a==b;"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1088::TRACK_PROOF, 0x52540000 | 1088);
}
