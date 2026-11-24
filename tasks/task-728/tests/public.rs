use task_728::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_pipe(Some("first\n123")),
        Some(123)
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe(Some("header\n猫|123")),
        None
    );
}
