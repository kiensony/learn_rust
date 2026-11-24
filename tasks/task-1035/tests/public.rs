use task_1035::*;

#[test]
fn public_examples() {
    assert_eq!(
        closing_marks_removed_equals(["72;12", "猫=12;note"]),
        [2, 6]
    );
    assert_eq!(
        closing_marks_removed_equals([String::from("猫=12;note"), String::from("hi=kit")]),
        [6, 6]
    );
}
