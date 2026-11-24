use task_1174::*;

#[test]
fn public_examples() {
    assert_eq!(closing_marks_removed_pipe("first\n123", "backup"), "123");
    assert_eq!(
        closing_marks_removed_pipe("header\n猫|123", "backup"),
        "猫|123"
    );
}
