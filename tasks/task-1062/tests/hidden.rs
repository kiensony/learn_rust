use task_1062::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_slash(&"hi/kit"), 3);
    assert_eq!(closing_field_slash(&String::from("/é/a")), 1);
    assert_eq!(closing_field_slash(&""), 0);
    assert_eq!(closing_field_slash(&String::from("/")), 0);
    assert_eq!(closing_field_slash(&"cat"), 3);
    assert_eq!(closing_field_slash(&String::from("//one/猫/ab")), 2);
    assert_eq!(closing_field_slash(&"aa/bb"), 2);
    assert_eq!(closing_field_slash(&String::from("ok/🐈")), 4);
    assert_eq!(closing_field_slash(&"path/first/last"), 4);
    assert_eq!(closing_field_slash(&String::from("path/path/")), 0);
    assert_eq!(closing_field_slash(&"path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1062::TRACK_PROOF, 0x52540000 | 1062);
}
