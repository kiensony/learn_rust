use task_1024::*;

#[test]
fn public_examples() {
    assert_eq!(
        leading_marks_removed_pipe(["first\n123", "header\n猫|123"]),
        [3, 7]
    );
    assert_eq!(
        leading_marks_removed_pipe([String::from("header\n猫|123"), String::from("hi|kit")]),
        [7, 6]
    );
}
