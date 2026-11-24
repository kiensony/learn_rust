use task_838::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_equals("72;12"),
        Err(FieldError::Missing)
    );
    assert_eq!(remainder_after_first_mark_equals("猫=12;note"), Ok(2));
}
