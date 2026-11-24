use task_818::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_slash("hi/kit"), Ok(3));
    assert_eq!(remainder_after_first_mark_slash("/é/a"), Ok(4));
    assert_eq!(
        remainder_after_first_mark_slash(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_slash("/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_slash("cat"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_slash("//one/猫/ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(remainder_after_first_mark_slash("aa/bb"), Ok(2));
    assert_eq!(remainder_after_first_mark_slash("ok/🐈"), Ok(4));
    assert_eq!(remainder_after_first_mark_slash("path/first/last"), Ok(4));
    assert_eq!(
        remainder_after_first_mark_slash("path/path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_slash("path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        remainder_after_first_mark_slash("abcdefghij"),
        Err(FieldError::Missing)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_818::TRACK_PROOF, 0x52540000 | 818);
}
