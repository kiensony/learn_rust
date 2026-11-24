use task_825::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_pipe("first\n123"), Ok(3));
    assert_eq!(
        leading_marks_removed_pipe("header\n猫|123"),
        Err(FieldError::TooLong)
    );
}
