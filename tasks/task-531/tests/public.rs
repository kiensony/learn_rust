use task_531::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_pipe("first\n123"), 3);
    assert_eq!(closing_marks_removed_pipe("header\n猫|123"), 5);
}
