use task_584::*;

#[test]
fn public_examples() {
    assert_eq!(first_nonempty_field_pipe("first\n123"), "123");
    assert_eq!(first_nonempty_field_pipe("header\n猫|123"), "猫");
}
