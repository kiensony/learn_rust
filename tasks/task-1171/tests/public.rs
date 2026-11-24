use task_1171::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe("first\n123", "backup"), "123");
    assert_eq!(leading_field_pipe("header\n猫|123", "backup"), "猫");
}
