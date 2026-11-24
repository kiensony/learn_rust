use task_814::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_slash("hi/kit"), Ok(3));
    assert_eq!(closing_field_slash("/é/a"), Ok(1));
    assert_eq!(closing_field_slash(""), Err(FieldError::Missing));
    assert_eq!(closing_field_slash("/"), Err(FieldError::Missing));
    assert_eq!(closing_field_slash("cat"), Ok(3));
    assert_eq!(closing_field_slash("//one/猫/ab"), Ok(2));
    assert_eq!(closing_field_slash("aa/bb"), Ok(2));
    assert_eq!(closing_field_slash("ok/🐈"), Ok(4));
    assert_eq!(closing_field_slash("path/first/last"), Ok(4));
    assert_eq!(closing_field_slash("path/path/"), Err(FieldError::Missing));
    assert_eq!(closing_field_slash("path/"), Err(FieldError::Missing));
    assert_eq!(closing_field_slash("abcdefghij"), Err(FieldError::TooLong));
}

#[test]
fn submission_marker() {
    assert_eq!(task_814::TRACK_PROOF, 0x52540000 | 814);
}
