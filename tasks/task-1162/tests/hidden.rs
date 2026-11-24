use task_1162::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_slash("hi/kit", "backup"), "kit");
    assert_eq!(closing_field_slash("/é/a", "backup"), "a");
    assert_eq!(closing_field_slash("", "backup"), "backup");
    assert_eq!(closing_field_slash("/", "backup"), "backup");
    assert_eq!(closing_field_slash("cat", "backup"), "cat");
    assert_eq!(closing_field_slash("//one/猫/ab", "backup"), "ab");
    assert_eq!(closing_field_slash("aa/bb", "backup"), "bb");
    assert_eq!(closing_field_slash("ok/🐈", "backup"), "🐈");
    assert_eq!(closing_field_slash("path/first/last", "backup"), "last");
    assert_eq!(closing_field_slash("path/path/", "backup"), "backup");
    assert_eq!(closing_field_slash("path/", "backup"), "backup");
}

#[test]
fn submission_marker() {
    assert_eq!(task_1162::TRACK_PROOF, 0x52540000 | 1162);
}
