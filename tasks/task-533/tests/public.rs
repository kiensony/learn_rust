use task_533::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_pipe("first\n123"), 0);
    assert_eq!(remainder_after_first_mark_pipe("header\n猫|123"), 3);
}
