use task_1040::*;

#[test]
fn public_examples() {
    assert_eq!(
        longest_field_with_last_tie_equals(["72;12", "猫=12;note"]),
        [2, 3]
    );
    assert_eq!(
        longest_field_with_last_tie_equals([String::from("猫=12;note"), String::from("hi=kit")]),
        [3, 3]
    );
}
