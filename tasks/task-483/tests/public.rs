use task_483::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_pipe(("first\n123", "x")), 0);
    assert_eq!(
        remainder_after_first_mark_pipe(("header\n猫|123", "z|w")),
        3
    );
}
