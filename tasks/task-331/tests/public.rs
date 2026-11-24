use task_331::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_pipe(String::from("first\n123")),
        "123"
    );
    assert_eq!(
        closing_marks_removed_pipe(String::from("header\n猫|123")),
        "猫|123"
    );
}
