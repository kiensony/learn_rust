use task_816::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_slash("hi/kit"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_slash("/é/a"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_slash(""), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_slash("/"), Err(FieldError::Missing));
    assert_eq!(closing_marks_removed_slash("cat"), Ok(3));
    assert_eq!(
        closing_marks_removed_slash("//one/猫/ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_slash("aa/bb"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_slash("ok/🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        closing_marks_removed_slash("path/first/last"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_slash("path/path/"), Ok(4));
    assert_eq!(
        closing_marks_removed_slash("path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        closing_marks_removed_slash("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_816::TRACK_PROOF, 0x52540000 | 816);
}
