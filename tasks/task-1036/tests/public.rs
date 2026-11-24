use task_1036::*;

#[test]
fn public_examples() {
    assert_eq!(
        marks_removed_at_both_ends_equals(["72;12", "猫=12;note"]),
        [2, 6]
    );
    assert_eq!(
        marks_removed_at_both_ends_equals([String::from("猫=12;note"), String::from("hi=kit")]),
        [6, 6]
    );
}
