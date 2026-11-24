use task_577::*;

#[test]
fn public_examples() {
    assert_eq!(leading_field_pipe("first\n123"), "123");
    assert_eq!(leading_field_pipe("header\n猫|123"), "猫");
}
