use task_1173::*;

#[test]
fn public_examples() {
    assert_eq!(leading_marks_removed_pipe("first\n123", "backup"), "123");
    assert_eq!(
        leading_marks_removed_pipe("header\n猫|123", "backup"),
        "猫|123"
    );
}
