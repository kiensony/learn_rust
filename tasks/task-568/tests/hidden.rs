use task_568::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_slash("hi/kit"), "kit");
    assert_eq!(closing_field_slash("/é/a"), "a");
    assert_eq!(closing_field_slash(""), "");
    assert_eq!(closing_field_slash("/"), "");
    assert_eq!(closing_field_slash("cat"), "cat");
    assert_eq!(closing_field_slash("//one/猫/ab"), "ab");
    assert_eq!(closing_field_slash("aa/bb"), "bb");
    assert_eq!(closing_field_slash("ok/🐈"), "🐈");
    assert_eq!(closing_field_slash("path/first/last"), "last");
    assert_eq!(closing_field_slash("path/path/"), "");
    assert_eq!(closing_field_slash("path/"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_568::TRACK_PROOF, 0x52540000 | 568);
}
