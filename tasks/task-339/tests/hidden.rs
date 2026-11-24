use task_339::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_equals(String::from("hi=kit")), "kit");
    assert_eq!(closing_field_equals(String::from("=é=a")), "a");
    assert_eq!(closing_field_equals(String::from("")), "");
    assert_eq!(closing_field_equals(String::from("=")), "");
    assert_eq!(closing_field_equals(String::from("cat")), "cat");
    assert_eq!(closing_field_equals(String::from("==one=猫=ab")), "ab");
    assert_eq!(closing_field_equals(String::from("aa=bb")), "bb");
    assert_eq!(closing_field_equals(String::from("ok=🐈")), "🐈");
    assert_eq!(closing_field_equals(String::from("aa=bb;meta=123")), "bb");
    assert_eq!(closing_field_equals(String::from(";ignore=all")), "");
    assert_eq!(closing_field_equals(String::from("a==b;")), "b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_339::TRACK_PROOF, 0x52540000 | 339);
}
