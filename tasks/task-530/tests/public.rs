use task_530::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_pipe("first\n123"), 3);
    assert_eq!(leading_marks_removed_pipe("header\n猫|123"), 5);
}
