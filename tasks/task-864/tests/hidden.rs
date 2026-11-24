use task_864::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_slash("hi/kit"), 5);
    assert_eq!(closing_field_slash("/é/a"), 3);
    assert_eq!(closing_field_slash(""), 2);
    assert_eq!(closing_field_slash("/"), 2);
    assert_eq!(closing_field_slash("cat"), 5);
    assert_eq!(closing_field_slash("//one/猫/ab"), 4);
    assert_eq!(closing_field_slash("aa/bb"), 4);
    assert_eq!(closing_field_slash("ok/🐈"), 6);
    assert_eq!(closing_field_slash("path/first/last"), 6);
    assert_eq!(closing_field_slash("path/path/"), 2);
    assert_eq!(closing_field_slash("path/"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_864::TRACK_PROOF, 0x52540000 | 864);
}
