use task_388::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_equals(&String::from("hi=kit")), 2);
    assert_eq!(leading_field_equals(&String::from("=é=a")), 0);
    assert_eq!(leading_field_equals(&String::from("")), 0);
    assert_eq!(leading_field_equals(&String::from("=")), 0);
    assert_eq!(leading_field_equals(&String::from("cat")), 3);
    assert_eq!(leading_field_equals(&String::from("==one=猫=ab")), 0);
    assert_eq!(leading_field_equals(&String::from("aa=bb")), 2);
    assert_eq!(leading_field_equals(&String::from("ok=🐈")), 2);
    assert_eq!(leading_field_equals(&String::from("aa=bb;meta=123")), 2);
    assert_eq!(leading_field_equals(&String::from(";ignore=all")), 0);
    assert_eq!(leading_field_equals(&String::from("a==b;")), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_388::TRACK_PROOF, 0x52540000 | 388);
}
