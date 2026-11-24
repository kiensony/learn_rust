use task_519::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_slash("hi/kit"), 3);
    assert_eq!(closing_field_slash("/é/a"), 1);
    assert_eq!(closing_field_slash(""), 0);
    assert_eq!(closing_field_slash("/"), 0);
    assert_eq!(closing_field_slash("cat"), 3);
    assert_eq!(closing_field_slash("//one/猫/ab"), 2);
    assert_eq!(closing_field_slash("aa/bb"), 2);
    assert_eq!(closing_field_slash("ok/🐈"), 1);
    assert_eq!(closing_field_slash("path/first/last"), 4);
    assert_eq!(closing_field_slash("path/path/"), 0);
    assert_eq!(closing_field_slash("path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_519::TRACK_PROOF, 0x52540000 | 519);
}
