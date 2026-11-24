use task_328::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe(String::from("first\n123")), "123");
    assert_eq!(leading_field_pipe(String::from("header\n猫|123")), "猫");
}
