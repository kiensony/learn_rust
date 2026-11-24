use task_579::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_pipe("first\n123"), "123");
    assert_eq!(leading_marks_removed_pipe("header\n猫|123"), "猫|123");
}
