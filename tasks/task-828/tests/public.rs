use task_828::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe("first\n123"),
        Err(FieldError::Missing)
    );
    assert_eq!(remainder_after_first_mark_pipe("header\n猫|123"), Ok(3));
}
