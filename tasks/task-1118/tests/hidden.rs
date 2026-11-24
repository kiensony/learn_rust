use task_1118::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_slash(&"hi/kit"), 2);
    assert_eq!(first_nonempty_field_slash(&String::from("/é/a")), 2);
    assert_eq!(first_nonempty_field_slash(&""), 0);
    assert_eq!(first_nonempty_field_slash(&String::from("/")), 0);
    assert_eq!(first_nonempty_field_slash(&"cat"), 3);
    assert_eq!(first_nonempty_field_slash(&String::from("//one/猫/ab")), 3);
    assert_eq!(first_nonempty_field_slash(&"aa/bb"), 2);
    assert_eq!(first_nonempty_field_slash(&String::from("ok/🐈")), 2);
    assert_eq!(first_nonempty_field_slash(&"path/first/last"), 5);
    assert_eq!(first_nonempty_field_slash(&String::from("path/path/")), 4);
    assert_eq!(first_nonempty_field_slash(&"path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1118::TRACK_PROOF, 0x52540000 | 1118);
}
