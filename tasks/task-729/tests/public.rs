use task_729::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_pipe(Some("first\n123")), None);
    assert_eq!(
        remainder_after_first_mark_pipe(Some("header\n猫|123")),
        Some(123)
    );
}
