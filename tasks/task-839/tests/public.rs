use task_839::*;

#[test]
fn public_examples() {
    assert_eq!(
        prefix_before_final_mark_equals("72;12"),
        Err(FieldError::Missing)
    );
    assert_eq!(prefix_before_final_mark_equals("猫=12;note"), Ok(3));
}
