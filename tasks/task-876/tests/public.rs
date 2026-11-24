use task_876::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_pipe("first\n123"), 5);
    assert_eq!(closing_marks_removed_pipe("header\n猫|123"), 9);
}
