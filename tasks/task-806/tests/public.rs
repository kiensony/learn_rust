use task_806::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_colon("hi:kit"),
        Err(FieldError::TooLong)
    );
    assert_eq!(closing_marks_removed_colon(""), Err(FieldError::Missing));
}
