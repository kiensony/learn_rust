use task_807::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_colon("hi:kit"),
        Err(FieldError::TooLong)
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(""),
        Err(FieldError::Missing)
    );
}
