use task_820::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_slash("hi/kit"), Ok(2));
    assert_eq!(first_nonempty_field_slash("/é/a"), Ok(2));
    assert_eq!(first_nonempty_field_slash(""), Err(FieldError::Missing));
    assert_eq!(first_nonempty_field_slash("/"), Err(FieldError::Missing));
    assert_eq!(first_nonempty_field_slash("cat"), Ok(3));
    assert_eq!(first_nonempty_field_slash("//one/猫/ab"), Ok(3));
    assert_eq!(first_nonempty_field_slash("aa/bb"), Ok(2));
    assert_eq!(first_nonempty_field_slash("ok/🐈"), Ok(2));
    assert_eq!(
        first_nonempty_field_slash("path/first/last"),
        Err(FieldError::TooLong)
    );
    assert_eq!(first_nonempty_field_slash("path/path/"), Ok(4));
    assert_eq!(
        first_nonempty_field_slash("path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        first_nonempty_field_slash("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_820::TRACK_PROOF, 0x52540000 | 820);
}
