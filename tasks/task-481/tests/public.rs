use task_481::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_pipe(("first\n123", "x")), 3);
    assert_eq!(closing_marks_removed_pipe(("header\n猫|123", "z|w")), 7);
}
