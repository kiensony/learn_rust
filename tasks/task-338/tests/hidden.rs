use task_338::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_equals(String::from("hi=kit")), "hi");
    assert_eq!(leading_field_equals(String::from("=é=a")), "");
    assert_eq!(leading_field_equals(String::from("")), "");
    assert_eq!(leading_field_equals(String::from("=")), "");
    assert_eq!(leading_field_equals(String::from("cat")), "cat");
    assert_eq!(leading_field_equals(String::from("==one=猫=ab")), "");
    assert_eq!(leading_field_equals(String::from("aa=bb")), "aa");
    assert_eq!(leading_field_equals(String::from("ok=🐈")), "ok");
    assert_eq!(leading_field_equals(String::from("aa=bb;meta=123")), "aa");
    assert_eq!(leading_field_equals(String::from(";ignore=all")), "");
    assert_eq!(leading_field_equals(String::from("a==b;")), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_338::TRACK_PROOF, 0x52540000 | 338);
}
