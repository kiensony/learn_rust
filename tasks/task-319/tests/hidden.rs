use task_319::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_slash(String::from("hi/kit")), "kit");
    assert_eq!(closing_field_slash(String::from("/é/a")), "a");
    assert_eq!(closing_field_slash(String::from("")), "");
    assert_eq!(closing_field_slash(String::from("/")), "");
    assert_eq!(closing_field_slash(String::from("cat")), "cat");
    assert_eq!(closing_field_slash(String::from("//one/猫/ab")), "ab");
    assert_eq!(closing_field_slash(String::from("aa/bb")), "bb");
    assert_eq!(closing_field_slash(String::from("ok/🐈")), "🐈");
    assert_eq!(closing_field_slash(String::from("path/first/last")), "last");
    assert_eq!(closing_field_slash(String::from("path/path/")), "");
    assert_eq!(closing_field_slash(String::from("path/")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_319::TRACK_PROOF, 0x52540000 | 319);
}
