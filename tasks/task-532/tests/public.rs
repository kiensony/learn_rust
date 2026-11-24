use task_532::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_pipe("first\n123"), 3);
    assert_eq!(marks_removed_at_both_ends_pipe("header\n猫|123"), 5);
}
