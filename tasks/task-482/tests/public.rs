use task_482::*;

#[test]
fn public_examples() {
    assert_eq!(marks_removed_at_both_ends_pipe(("first\n123", "x")), 3);
    assert_eq!(
        marks_removed_at_both_ends_pipe(("header\n猫|123", "z|w")),
        7
    );
}
