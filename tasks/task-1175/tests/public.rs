use task_1175::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_pipe("first\n123", "backup"),
        "123"
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe("header\n猫|123", "backup"),
        "猫|123"
    );
}
