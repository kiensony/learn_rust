use task_1033::*;

#[test]
fn public_examples() {
    assert_eq!(closing_field_equals(["72;12", "猫=12;note"]), [2, 2]);
    assert_eq!(
        closing_field_equals([String::from("猫=12;note"), String::from("hi=kit")]),
        [2, 3]
    );
}
