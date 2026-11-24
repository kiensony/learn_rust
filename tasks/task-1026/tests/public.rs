use task_1026::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_pipe(["first\n123", "header\n猫|123"]),
        [3, 7]
    );
    assert_eq!(
        marks_removed_at_both_ends_pipe([String::from("header\n猫|123"), String::from("hi|kit")]),
        [7, 6]
    );
}
