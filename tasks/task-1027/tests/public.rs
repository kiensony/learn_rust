use task_1027::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_pipe(["first\n123", "header\n猫|123"]),
        [0, 3]
    );
    assert_eq!(
        remainder_after_first_mark_pipe([String::from("header\n猫|123"), String::from("hi|kit")]),
        [3, 3]
    );
}
