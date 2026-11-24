use task_1176::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe("first\n123", "backup"),
        "backup"
    );
    assert_eq!(
        remainder_after_first_mark_pipe("header\n猫|123", "backup"),
        "123"
    );
}
