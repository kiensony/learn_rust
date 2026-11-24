use task_727::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_pipe(Some("first\n123")), Some(123));
    assert_eq!(closing_marks_removed_pipe(Some("header\n猫|123")), None);
}
