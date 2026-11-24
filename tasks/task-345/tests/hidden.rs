use task_345::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_equals(String::from("hi=kit")), "hi");
    assert_eq!(first_nonempty_field_equals(String::from("=é=a")), "é");
    assert_eq!(first_nonempty_field_equals(String::from("")), "");
    assert_eq!(first_nonempty_field_equals(String::from("=")), "");
    assert_eq!(first_nonempty_field_equals(String::from("cat")), "cat");
    assert_eq!(
        first_nonempty_field_equals(String::from("==one=猫=ab")),
        "one"
    );
    assert_eq!(first_nonempty_field_equals(String::from("aa=bb")), "aa");
    assert_eq!(first_nonempty_field_equals(String::from("ok=🐈")), "ok");
    assert_eq!(
        first_nonempty_field_equals(String::from("aa=bb;meta=123")),
        "aa"
    );
    assert_eq!(first_nonempty_field_equals(String::from(";ignore=all")), "");
    assert_eq!(first_nonempty_field_equals(String::from("a==b;")), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_345::TRACK_PROOF, 0x52540000 | 345);
}
