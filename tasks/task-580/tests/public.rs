use task_580::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_pipe("first\n123"), "123");
    assert_eq!(closing_marks_removed_pipe("header\n猫|123"), "猫|123");
}
