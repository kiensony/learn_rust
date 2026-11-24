use task_817::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_slash("hi/kit"),
        Err(FieldError::TooLong)
    );
    assert_eq!(marks_removed_at_both_ends_slash("/é/a"), Ok(4));
    assert_eq!(
        marks_removed_at_both_ends_slash(""),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("/"),
        Err(FieldError::Missing)
    );
    assert_eq!(marks_removed_at_both_ends_slash("cat"), Ok(3));
    assert_eq!(
        marks_removed_at_both_ends_slash("//one/猫/ab"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("aa/bb"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("ok/🐈"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("path/first/last"),
        Err(FieldError::TooLong)
    );
    assert_eq!(marks_removed_at_both_ends_slash("path/path/"), Ok(4));
    assert_eq!(
        marks_removed_at_both_ends_slash("path/"),
        Err(FieldError::Missing)
    );
    assert_eq!(
        marks_removed_at_both_ends_slash("abcdefghij"),
        Err(FieldError::TooLong)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_817::TRACK_PROOF, 0x52540000 | 817);
}
