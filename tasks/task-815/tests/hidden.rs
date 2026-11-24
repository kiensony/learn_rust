use task_815::*;

#[test]
fn edge_cases() {
    assert_eq!(
        leading_marks_removed_slash("hi/kit"),
        Err(FieldError::TooLong)
    );
    assert_eq!(leading_marks_removed_slash("/é/a"), Ok(4));
    assert_eq!(leading_marks_removed_slash(""), Err(FieldError::Missing));
    assert_eq!(leading_marks_removed_slash("/"), Err(FieldError::Missing));
    assert_eq!(leading_marks_removed_slash("cat"), Ok(3));
    assert_eq!(
        leading_marks_removed_slash("//one/猫/ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        leading_marks_removed_slash("aa/bb"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        leading_marks_removed_slash("ok/🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        leading_marks_removed_slash("path/first/last"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        leading_marks_removed_slash("path/path/"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        leading_marks_removed_slash("path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        leading_marks_removed_slash("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_815::TRACK_PROOF, 0x52540000 | 815);
}
