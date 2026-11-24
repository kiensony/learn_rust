use task_878::*;

#[test]
fn public_examples() {
    assert_eq!(remainder_after_first_mark_pipe("first\n123"), 2);
    assert_eq!(remainder_after_first_mark_pipe("header\n猫|123"), 5);
}
